use std::time::Duration;

use moka::{Expiry, future::Cache};
use rocket::{
    Request,
    request::{self, FromRequest},
};
use rocket_okapi::OpenApiFromRequest;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, from_value, to_value};

use crate::{ApiError, ApiResult, Error};

#[derive(Clone, Debug)]
pub struct Expiration {
    pub lifetime: Option<chrono::Duration>,
    pub idletime: Option<chrono::Duration>,
}

impl Expiration {
    pub fn lifetime(&self) -> Option<Duration> {
        if let Some(lifetime) = self.lifetime {
            Some(lifetime.to_std().expect("POSITIVE CACHE TTL"))
        } else {
            None
        }
    }

    pub fn idletime(&self) -> Option<Duration> {
        if let Some(idle) = self.idletime {
            Some(idle.to_std().expect("POSITIVE CACHE TTL"))
        } else {
            None
        }
    }

    pub fn never() -> Self {
        Self {
            lifetime: None,
            idletime: None,
        }
    }
}

struct ExpirationResolver;

impl Expiry<String, (Expiration, Value)> for ExpirationResolver {
    fn expire_after_create(
        &self,
        _key: &String,
        value: &(Expiration, Value),
        _created_at: std::time::Instant,
    ) -> Option<Duration> {
        value.0.lifetime()
    }

    fn expire_after_read(
        &self,
        _key: &String,
        value: &(Expiration, Value),
        _read_at: std::time::Instant,
        duration_until_expiry: Option<Duration>,
        _last_modified_at: std::time::Instant,
    ) -> Option<Duration> {
        if let Some(idle) = value.0.idletime() {
            Some(idle)
        } else {
            duration_until_expiry
        }
    }
}

#[derive(Clone, Debug, OpenApiFromRequest)]
pub struct ResponseCache {
    cache: Cache<String, (Expiration, Value)>,
    default_expiration: Expiration,
}

impl ResponseCache {
    pub fn new(default_expiration: Expiration) -> Self {
        let expiry = ExpirationResolver;
        let cache = Cache::<String, (Expiration, Value)>::builder()
            .expire_after(expiry)
            .build();
        Self {
            cache,
            default_expiration,
        }
    }

    pub async fn cache_request<
        R: Send + Sync + Serialize + DeserializeOwned,
        F: Future<Output = ApiResult<R>>,
    >(
        &self,
        key: impl AsRef<str>,
        initializer: F,
        expiration: Option<Expiration>,
    ) -> ApiResult<R> {
        let key: String = key.as_ref().to_string();
        let default_expr = self.default_expiration.clone();
        let output = self
            .cache
            .try_get_with(key, async move {
                match initializer.await {
                    Ok(result) => to_value(result)
                        .or_else(|e| Err(ApiError::from(Error::SerializationError(e.to_string()))))
                        .and_then(|r| Ok((expiration.unwrap_or(default_expr), r))),
                    Err(e) => Err(e),
                }
            })
            .await;

        match output {
            Ok((_, value)) => from_value::<R>(value)
                .or_else(|e| Err(ApiError::from(Error::SerializationError(e.to_string())))),
            Err(err) => Err(err.as_ref().clone()),
        }
    }
}

#[async_trait::async_trait]
impl<'r> FromRequest<'r> for ResponseCache {
    type Error = ApiError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        request::Outcome::Success(
            req.rocket()
                .state::<ResponseCache>()
                .expect("No response cache initialized.")
                .clone(),
        )
    }
}
