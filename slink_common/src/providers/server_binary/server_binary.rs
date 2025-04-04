use std::{collections::HashMap, fmt::Debug, path::PathBuf};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{types::minecraft::MinecraftVersion, Error, Res};

use super::{super::error::{ProviderError, ProviderType}, fabric::FabricServerBinaryVersion};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
    pub enum ServerBinaryVersion {
        Fabric(FabricServerBinaryVersion)
    }

#[async_trait::async_trait]
pub trait ServerBinaryProvider {
    fn components() -> Vec<String> where Self: Sized;
    fn name() -> String where Self: Sized;
    async fn get_components(minecraft_version: MinecraftVersion) -> Res<HashMap<String, Vec<ServerBinaryVersion>>> where Self: Sized;
    async fn install_to(minecraft_version: MinecraftVersion, components: HashMap<String, ServerBinaryVersion>, directory: PathBuf) -> Res<()> where Self: Sized;

    async fn get_latest_stable_component(minecraft_version: MinecraftVersion, component: &str) -> Res<ServerBinaryVersion> where Self: Sized {
        let components = Self::get_components(minecraft_version.clone()).await?;
        if let Some(versions) = components.get(&component.to_string()) {
            if let Some(latest) = versions.first() {
                Ok(latest.clone())
            } else {
                Err(Self::error(ProviderError::NoVersions { component: component.to_string(), mc_version: minecraft_version.id }))
            }
        } else {
            Err(Self::error(ProviderError::UnknownVersionComponent(component.to_string())))
        }
    }

    async fn get_latest_unstable_component(minecraft_version: MinecraftVersion, component: &str) -> Res<ServerBinaryVersion> where Self: Sized {
        Self::get_latest_stable_component(minecraft_version, component).await
    }

    fn result<T: Sized>(res: Result<T, ProviderError>) -> Res<T> where Self: Sized {
        res.or_else(|e| Err(Error::provider_error(ProviderType::ServerBinary, Self::name(), e)))
    }

    fn error(err: ProviderError) -> Error where Self: Sized{
        Error::provider_error(ProviderType::ServerBinary, Self::name(), err)
    }

    fn id() -> String where Self: Sized {
        format!("providers/server_binary/{}", Self::name())
    }
}
