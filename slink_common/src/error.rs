use std::fmt::Debug;

use okapi::openapi3::{RefOr, Response as OpenApiResponse, Responses};
use reqwest::StatusCode;
use rocket::{Request, request, response::Responder, serde::json::Json};
use rocket_okapi::response::OpenApiResponderInner;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::providers::error::{ProviderError, ProviderType};

macro_rules! response {
    ($target:ident, $code:literal, $desc:expr) => {
        $target.insert(
            $code.to_string(),
            RefOr::Object(OpenApiResponse {
                description: concat!(
                    "# [Error ",
                    $code,
                    "](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/",
                    $code,
                    ")\n",
                    $desc
                )
                .to_string(),
                ..Default::default()
            }),
        );
    };
}

#[derive(thiserror::Error, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum Error {
    #[error("An unexpected error occurred: {0}")]
    Unexpected(String),

    #[error("A process runner ({scope}:{runner}) with ID {id} failed: {reason}")]
    RunnerError {
        scope: String,
        runner: String,
        id: String,
        reason: String,
    },

    #[error("The provided value <{value}> was invalid: {reason}")]
    ValueError { value: String, reason: String },

    #[error("An error occured in the {provider_type}.{provider_name}: {error:?}")]
    ProviderError {
        provider_type: ProviderType,
        provider_name: String,
        error: ProviderError,
    },

    #[error("Encountered an error dispatching a network request: {0}")]
    RequestDispatchError(String),

    #[error("Received an error response from {url} with status {status}: {reason}")]
    RequestResponseError {
        url: String,
        status: String,
        reason: String,
    },

    #[error("Failed to parse response data: {0}")]
    ResponseParsingError(String),

    #[error("Failed to get {path} in {data}: {reason}")]
    DataPathError {
        path: String,
        data: String,
        reason: String
    },

    #[error("Failed to deserialize input: {0}")]
    DeserializationError(String),

    #[error("Failed to serialize input: {0}")]
    SerializationError(String)
}

impl Error {
    pub fn unexpected<T>(err: impl std::error::Error) -> Res<T> {
        Err(Self::Unexpected(err.to_string()))
    }

    pub fn value_error(value: impl Debug, error: impl Debug) -> Self {
        Self::ValueError {
            value: format!("{value:?}"),
            reason: format!("{error:?}"),
        }
    }

    pub fn provider_error(
        provider_type: ProviderType,
        provider_name: impl AsRef<str>,
        error: ProviderError,
    ) -> Self {
        Self::ProviderError {
            provider_type,
            provider_name: provider_name.as_ref().to_string(),
            error,
        }
    }

    pub fn response(result: Result<reqwest::Response, reqwest::Error>) -> Res<reqwest::Response> {
        result.or_else(|e| {
            Err(Self::RequestDispatchError(e.to_string()))
        })?.error_for_status().or_else(|e| {
            Err(Self::RequestResponseError {
                url: e
                    .url()
                    .and_then(|u| Some(u.to_string()))
                    .unwrap_or(String::from("UNKNOWN URL")),
                status: e
                    .status()
                    .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
                    .to_string(),
                reason: e.to_string(),
            })
        })
    }

    pub async fn response_as<T: DeserializeOwned>(result: Result<reqwest::Response, reqwest::Error>) -> Res<T> {
        Self::response(result)?.json::<T>().await.or_else(|e| Err(Self::ResponseParsingError(e.to_string())))
    }

    pub fn deserialization(error: impl Debug) -> Self {
        Self::DeserializationError(format!("{error:?}"))
    }

    pub fn serialization(error: impl Debug) -> Self {
        Self::SerializationError(format!("{error:?}"))
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
    #[schemars(with = "Error")]
    #[response(status = 500)]
    Internal(Json<Error>),

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
    MissingAuthorization(String),
}

impl ApiError {
    pub fn configuration(reason: impl Into<String>) -> Self {
        Self::Configuration(reason.into())
    }

    pub fn respond<'r, T>(&self, request: &'r Request) -> request::Outcome<T, Self> {
        request.local_cache(|| self.clone());
        let status = self
            .clone()
            .respond_to(request)
            .and_then(|r| Ok(r.status()))
            .or_else(|s| Ok::<_, Self>(s))
            .unwrap();
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
        Self::Internal(Json(value))
    }
}

impl OpenApiResponderInner for ApiError {
    fn responses(
        _: &mut rocket_okapi::r#gen::OpenApiGenerator,
    ) -> rocket_okapi::Result<okapi::openapi3::Responses> {
        let mut items = rocket_okapi::okapi::schemars::Map::new();
        response!(
            items,
            400,
            "An error occurred while trying to parse the user's request."
        );
        response!(items, 404, "Requested resource not found");
        response!(
            items,
            500,
            "Internal server error occurred while processing request."
        );
        response!(
            items,
            401,
            "User is not authorized to perform this request."
        );

        Ok(Responses {
            responses: items,
            ..Default::default()
        })
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
