use rocket::{request, response::Responder, serde::json::Json, Request};
use serde::{Deserialize, Serialize};

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
    }
}

impl Error {
    pub fn unexpected<T>(err: impl std::error::Error) -> Res<T> {
        Err(Self::Unexpected(err.to_string()))
    }
}

pub type Res<T> = Result<T, Error>;


#[derive(thiserror::Error, Clone, Debug, Responder)]
#[response(content_type = "application/json")]
pub enum ApiError {
    #[error("Encountered an uncaught error: {0:?}")]
    #[response(status = 500)]
    Uncaught(String),

    #[error("Encountered an unexpected internal error: {0:?}")]
    #[response(status = 500)]
    Internal(Json<Error>),

    #[error("Configuration error: {0}")]
    #[response(status = 500)]
    Configuration(String)
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
}

impl From<Error> for ApiError {
    fn from(value: Error) -> Self {
        Self::Internal(Json(value))
    }
}

pub type ApiResult<T> = Result<T, ApiError>;