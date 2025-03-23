use chrono::{DateTime, Utc};
use manor::{Link, schema};
use rocket::request::{self, FromRequest};
use serde::{Deserialize, Serialize};
use slink_common::{ApiError, ApiResult};
use bson::Uuid;
use ts_rs::TS;
use crate::util::types::TSLink;

use crate::util::security::HashedPassword;

#[schema(collection = "sessions")]
#[derive(TS)]
#[ts(export)]
pub struct Session {
    #[field(id = Uuid::new)]
    pub id: Uuid,

    pub created: DateTime<Utc>,
    pub last_connection: DateTime<Utc>,

    #[serde(default)]
    #[ts(as = "TSLink")]
    pub user: Option<Link<User>>,
}

impl Session {
    pub fn create() -> Self {
        Session {
            id: Uuid::new(),
            created: Utc::now(),
            last_connection: Utc::now(),
            user: None,
            _collection: None
        }
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for Session {
    type Error = ApiError;
    async fn from_request(request: &'r rocket::Request<'_>) -> request::Outcome<Session, Self::Error> {
        let session: Option<Session> = request.local_cache(|| None::<Session>).clone();
        if let Some(sess) = session {
            request::Outcome::Success(sess)
        } else {
            ApiError::Uncaught("A session was not initialized in the local cache. This indicates a bug in the server software.".to_string()).respond(request)
        }
    }
}

#[schema(collection = "users")]
pub struct User {
    #[field(id = Uuid::new)]
    pub id: Uuid,
    pub username: String,
    pub hashed_password: HashedPassword,
}

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
#[ts(export)]
pub struct RedactedUser {
    pub id: Uuid,
    pub username: String
}

impl User {
    pub fn create(username: impl Into<String>, password: impl Into<String>) -> ApiResult<Self> {
        Ok(Self {
            id: Uuid::new(),
            username: username.into(),
            hashed_password: HashedPassword::new(password)?,
            _collection: None,
        })
    }

    pub fn redact(&self) -> RedactedUser {
        RedactedUser { id: self.id.clone(), username: self.username.clone() }
    }
}
