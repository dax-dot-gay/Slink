use std::collections::HashMap;

use reqwest::{Client, ClientBuilder};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};

use crate::{providers::error::ProviderError, utilities::get_at_path, Res, USER_AGENT};

use super::server_binary::ServerBinaryProvider;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, Eq, PartialEq)]
#[serde(tag = "component", rename_all = "snake_case")]
pub enum FabricServerBinaryVersion {
    Loader { version: String, stable: bool },
    Installer { version: String, stable: bool },
}

impl FabricServerBinaryVersion {
    pub fn version(&self) -> String {
        match self {
            Self::Loader { version, .. } => version.clone(),
            Self::Installer { version, .. } => version.clone(),
        }
    }

    pub fn kind(&self) -> String {
        match self {
            Self::Loader { .. } => "loader",
            Self::Installer { .. } => "installer",
        }
        .to_string()
    }
}

pub struct FabricServerBinaryProvider;

#[async_trait::async_trait]
impl ServerBinaryProvider for FabricServerBinaryProvider {
    type VersionComponent = FabricServerBinaryVersion;
    fn name() -> String
    where
        Self: Sized,
    {
        String::from("fabric")
    }

    fn components() -> Vec<String>
    where
        Self: Sized,
    {
        vec![String::from("loader"), String::from("installer")]
    }

    async fn get_components(
        minecraft_version: MinecraftVersion,
    ) -> Res<HashMap<String, Vec<Self::VersionComponent>>>
    where
        Self: Sized,
    {
        let loader_versions = Self::result(
            ProviderError::response_as::<Value>(
                Self::client()
                    .get(format!(
                        "https://meta.fabricmc.net/v2/versions/loader/{}",
                        minecraft_version.id
                    ))
                    .send()
                    .await,
            )
            .await,
        )?;
        let installer_versions = Self::result(
            ProviderError::response_as::<Value>(
                Self::client()
                    .get("https://meta.fabricmc.net/v2/versions/installer")
                    .send()
                    .await,
            )
            .await,
        )?;

        let parsed_loaders = loader_versions.as_array().ok_or(ProviderError::ResponseDataError(String::from("Invalid response shape"))).and_then(|v| v.iter().filter_map(|i| {
            let version = i.get("version").and_then(|r| r.as_str());
            let stable = i.get("stable").and_then(|r| r.as_bool());
        }))
    }
    async fn install_to(
        minecraft_version: MinecraftVersion,
        components: HashMap<String, Self::VersionComponent>,
        directory: impl AsRef<Path>,
    ) -> Res<()>
    where
        Self: Sized;
}

impl FabricServerBinaryProvider {
    fn client() -> Client {
        ClientBuilder::new()
            .user_agent(format!(
                "{} {}",
                USER_AGENT,
                FabricServerBinaryProvider::id()
            ))
            .build()
            .unwrap()
    }
}
