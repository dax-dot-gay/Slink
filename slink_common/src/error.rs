use std::fmt::Debug;

use okapi::openapi3::{RefOr, Responses, Response as OpenApiResponse};
use rocket::{request, response::Responder, Request};
use rocket_okapi::response::OpenApiResponderInner;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

macro_rules! response {
    ($target:ident, $code:literal, $desc:expr) => {
        $target.insert(
            $code.to_string(),
            RefOr::Object(OpenApiResponse {
                description: concat!("# [Error ", $code, "](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/", $code, ")\n", $desc).to_string(),
                ..Default::default()
            })
        );
    };
}

#[derive(thiserror::Error, Clone, Debug, Serialize, Deserialize)]
pub enum Error {
    #[error("An unexpected error occurred: {0}")]
    Unexpected(String),

    #[error("A process runner ({scope}:{runner}) with ID {id} failed: {reason}")]
    RunnerError {
        scope: String,
        runner: String,
        id: String,
        reason: String
    },

    #[error("The provided value <{value}> was invalid: {reason}")]
    ValueError {
        value: String,
        reason: String
    }
}

impl Error {
    pub fn unexpected<T>(err: impl std::error::Error) -> Res<T> {
        Err(Self::Unexpected(err.to_string()))
    }

    pub fn value_error(value: impl Debug, error: impl std::error::Error) -> Self {
        Self::ValueError { value: format!("{value:?}"), reason: error.to_string() }
    }
}

pub type Res<T> = Result<T, Error>;


#[derive(thiserror::Error, Clone, Debug, Responder, JsonSchema)]
#[response(content_type = "application/json")]
pub enum ApiError {
    #[error("Encountered an uncaught error: {0:?}")]
    #[response(status = 500)]
    Uncaught(String),

    #[error("Encountered an unexpected internal error: {0:?}")]
    #[response(status = 500)]
    Internal(String),

    #[error("Configuration error: {0}")]
    #[response(status = 500)]
    Configuration(String),

    #[error("Encountered a cryptographic error: {0}")]
    #[response(status = 400)]
    CryptographicError(String),

    #[error("Incorrect username or password")]
    #[response(status = 404)]
    BadLogin(()),

    #[error("Missing authorization resources: {0}")]
    #[response(status = 401)]
    MissingAuthorization(String)
}

impl ApiError {
    pub fn configuration(reason: impl Into<String>) -> Self {
        Self::Configuration(reason.into())
    }

    pub fn respond<'r, T>(&self, request: &'r Request) -> request::Outcome<T, Self> {
        request.local_cache(|| self.clone());
        let status = self.clone().respond_to(request).and_then(|r| Ok(r.status())).or_else(|s| Ok::<_, Self>(s)).unwrap();
        request::Outcome::<T, Self>::Error((status, self.clone()))
    }

    pub fn bad_login() -> Self {
        Self::BadLogin(())
    }

    pub fn missing_auth(context: impl AsRef<str>) -> Self {
        Self::MissingAuthorization(context.as_ref().to_string())
    }
}

impl From<Error> for ApiError {
    fn from(value: Error) -> Self {
        Self::Internal(value.to_string())
    }
}

impl OpenApiResponderInner for ApiError {
    fn responses(_: &mut rocket_okapi::r#gen::OpenApiGenerator) -> rocket_okapi::Result<okapi::openapi3::Responses> {
        let mut items = rocket_okapi::okapi::schemars::Map::new();
        response!(items, 400, "An error occurred while trying to parse the user's request.");
        response!(items, 404, "Requested resource not found");
        response!(items, 500, "Internal server error occurred while processing request.");
        response!(items, 401, "User is not authorized to perform this request.");

        Ok(Responses { responses: items, ..Default::default() })
    }
}

pub type ApiResult<T> = Result<T, ApiError>;