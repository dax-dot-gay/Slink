use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Error, Res, types::Version};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct FabricServerVersion {
    pub minecraft: String,
    pub loader: Version,
    pub installer: Version,
}

impl FabricServerVersion {
    pub fn url(&self) -> String {
        format!(
            "https://meta.fabricmc.net/v2/versions/loader/{minecraft}/{loader}/{installer}/server/jar",
            minecraft = self.minecraft,
            loader = self.loader.to_string(),
            installer = self.installer.to_string()
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct FabricApiMinecraftVersion {
    pub version: String,
    pub stable: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct FabricApiInstallerVersion {
    pub version: String,
    pub stable: bool,
    pub url: String,
    pub maven: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct FabricApiLoaderVersion {
    pub version: String,
    pub stable: bool,
    pub separator: String,
    pub build: u32,
    pub maven: String,
}

pub struct FabricApi;

impl FabricApi {
    pub async fn minecraft_versions() -> Res<Vec<FabricApiMinecraftVersion>> {
        let response = reqwest::get("https://meta.fabricmc.net/v2/versions/game")
            .await
            .or_else(|e| Err(Error::request_error(e)))?
            .error_for_status()
            .or_else(|e| Err(Error::request_error(e)))?;
        response
            .json()
            .await
            .or_else(|e| Err(Error::request_error(e)))
    }

    pub async fn installer_versions() -> Res<Vec<FabricApiInstallerVersion>> {
        let response = reqwest::get("https://meta.fabricmc.net/v2/versions/installer")
            .await
            .or_else(|e| Err(Error::request_error(e)))?
            .error_for_status()
            .or_else(|e| Err(Error::request_error(e)))?;
        response
            .json()
            .await
            .or_else(|e| Err(Error::request_error(e)))
    }

    pub async fn loader_versions() -> Res<Vec<FabricApiLoaderVersion>> {
        let response = reqwest::get("https://meta.fabricmc.net/v2/versions/loader")
            .await
            .or_else(|e| Err(Error::request_error(e)))?
            .error_for_status()
            .or_else(|e| Err(Error::request_error(e)))?;
        response
            .json()
            .await
            .or_else(|e| Err(Error::request_error(e)))
    }
}
