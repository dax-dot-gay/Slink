use std::fmt::Display;

use reqwest::{Response, StatusCode};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProviderType {
    ServerBinary,
}

impl Display for ProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::ServerBinary => "server_binary",
        })
    }
}

#[derive(thiserror::Error, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum ProviderError {
    #[error("Missing version component: {0}")]
    MissingVersionComponent(String),

    #[error("Error dispatching request to provider upstream: {0}")]
    RequestError(String),

    #[error("Got status code {status} from {url}: {data}")]
    ResponseError {
        status: String,
        url: String,
        data: String,
    },

    #[error("Failed to parse response data: {0}")]
    ResponseDataError(String),

    #[error("Unknown version component: {0}")]
    UnknownVersionComponent(String),

    #[error("No {component} versions for MCV {mc_version}")]
    NoVersions {
        component: String,
        mc_version: String
    },

    #[error("Failed to download to {path}: {reason}")]
    DownloadError {
        path: String,
        reason: String
    }
}

impl ProviderError {
    pub fn response(input: Result<Response, reqwest::Error>) -> Result<Response, Self> {
        input
            .or_else(|e| Err(Self::RequestError(e.to_string())))?
            .error_for_status()
            .or_else(|e| {
                Err(Self::ResponseError {
                    status: e
                        .status()
                        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
                        .to_string(),
                    url: e
                        .url()
                        .and_then(|u| Some(u.to_string()))
                        .unwrap_or(String::from("UNKNOWN_URL")),
                    data: e.to_string(),
                })
            })
    }

    pub async fn response_as<T: DeserializeOwned>(
        input: Result<Response, reqwest::Error>,
    ) -> Result<T, Self> {
        Self::response(input)?
            .json::<T>()
            .await
            .or_else(|e| Err(Self::ResponseDataError(e.to_string())))
    }
}
