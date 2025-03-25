use std::collections::HashMap;

use reqwest::ClientBuilder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Res, USER_AGENT};

use super::{super::error::ProviderError, server_binary::ServerBinary};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum FabricComponentType {
    Minecraft,
    Loader,
    Installer,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct FabricComponentVersion {
    pub index: u64,
    pub version: String,
    pub stable: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FabricApiComponentVersion {
    pub version: String,
    pub stable: bool,
}

impl Ord for FabricComponentVersion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Ord::cmp(&self.index, &other.index)
    }
}

impl PartialOrd for FabricComponentVersion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        PartialOrd::partial_cmp(&self.index, &other.index)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct FabricCompiledVersion {
    pub minecraft: FabricComponentVersion,
    pub loader: FabricComponentVersion,
    pub installer: FabricComponentVersion,
}

impl FabricCompiledVersion {
    pub fn url(&self) -> String {
        format!(
            "https://meta.fabricmc.net/v2/versions/loader/{minecraft}/{loader}/{installer}/server/jar",
            minecraft = self.minecraft.version,
            loader = self.loader.version,
            installer = self.installer.version
        )
    }
}

pub struct FabricServer;

impl FabricServer {
    fn client() -> reqwest::Client {
        ClientBuilder::new()
            .user_agent(format!("{} {}", USER_AGENT, "components/fabric_modloader"))
            .build()
            .unwrap()
    }
}

#[async_trait::async_trait]
impl ServerBinary for FabricServer {
    type ComponentType = FabricComponentType;
    type ComponentVersion = FabricComponentVersion;
    type CompiledVersion = FabricCompiledVersion;
    fn provider_name() -> String {
        String::from("fabric_modloader")
    }

    fn get_component_types() -> Vec<Self::ComponentType> {
        vec![
            FabricComponentType::Minecraft,
            FabricComponentType::Loader,
            FabricComponentType::Installer,
        ]
    }

    fn get_compiled_version(
        components: HashMap<Self::ComponentType, Self::ComponentVersion>,
    ) -> Res<Self::CompiledVersion>
    where
        Self: Sized,
    {
        let minecraft = components
            .get(&FabricComponentType::Minecraft)
            .ok_or(Self::wrap_error(ProviderError::MissingVersionComponent(
                String::from("minecraft"),
            )))?
            .clone();
        let loader = components
            .get(&FabricComponentType::Loader)
            .ok_or(Self::wrap_error(ProviderError::MissingVersionComponent(
                String::from("loader"),
            )))?
            .clone();
        let installer = components
            .get(&FabricComponentType::Installer)
            .ok_or(Self::wrap_error(ProviderError::MissingVersionComponent(
                String::from("installer"),
            )))?
            .clone();

        Ok(FabricCompiledVersion {
            minecraft,
            loader,
            installer,
        })
    }

    async fn get_component_versions()
    -> Res<HashMap<Self::ComponentType, Vec<Self::ComponentVersion>>>
    where
        Self: Sized,
    {
        let mut results: HashMap<FabricComponentType, Vec<FabricApiComponentVersion>> =
            HashMap::new();

        results.insert(
            FabricComponentType::Minecraft,
            Self::wrap_result(
                ProviderError::response_as::<Vec<FabricApiComponentVersion>>(
                    Self::client()
                        .get("https://meta.fabricmc.net/v2/versions/game")
                        .send()
                        .await,
                )
                .await,
            )?,
        );
        results.insert(
            FabricComponentType::Loader,
            Self::wrap_result(
                ProviderError::response_as::<Vec<FabricApiComponentVersion>>(
                    Self::client()
                        .get("https://meta.fabricmc.net/v2/versions/loader")
                        .send()
                        .await,
                )
                .await,
            )?,
        );
        results.insert(
            FabricComponentType::Installer,
            Self::wrap_result(
                ProviderError::response_as::<Vec<FabricApiComponentVersion>>(
                    Self::client()
                        .get("https://meta.fabricmc.net/v2/versions/installer")
                        .send()
                        .await,
                )
                .await,
            )?,
        );

        Ok(results
            .iter()
            .map(|(typ, versions)| {
                (
                    typ.clone(),
                    versions
                        .iter()
                        .enumerate()
                        .map(|(index, version)| FabricComponentVersion {
                            index: index as u64,
                            version: version.version.clone(),
                            stable: version.stable.clone(),
                        })
                        .collect::<Vec<FabricComponentVersion>>(),
                )
            })
            .collect())
    }

    async fn download_server(version: Self::CompiledVersion) -> Res<reqwest::Response>
    where
        Self: Sized,
    {
        Self::wrap_result(ProviderError::response(
            Self::client().get(version.url()).send().await,
        ))
    }
}
