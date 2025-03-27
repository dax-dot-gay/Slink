use std::{collections::HashMap, ops::Deref};

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strfmt::strfmt;

use crate::{utilities::get_one_at_path, Error, Res, JAVA_CONTAINER_BASE, MINECRAFT_VERSIONS_MANIFEST, USER_AGENT};

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq, Eq)]
pub struct JavaVersion(pub u8);

impl JavaVersion {
    pub fn version(&self) -> u8 {
        self.0
    }

    pub fn image(&self) -> String {
        let mut map = HashMap::<String, String>::new();
        map.insert(String::from("version"), self.0.to_string());
        strfmt(JAVA_CONTAINER_BASE, &map).unwrap()
    }
}

impl Deref for JavaVersion {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u8> for JavaVersion {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl Into<u8> for JavaVersion {
    fn into(self) -> u8 {
        self.0
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MinecraftVersionType {
    Release,

    #[serde(other)]
    Snapshot,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MinecraftVersion {
    pub id: String,

    #[serde(rename = "type")]
    pub version_type: MinecraftVersionType,
    pub url: String,
    pub time: DateTime<Utc>,

    #[serde(rename = "releaseTime")]
    pub release_ime: DateTime<Utc>,
    pub sha1: String,

    #[serde(rename = "complianceLevel")]
    pub compliance_level: u32
}

impl MinecraftVersion {
    pub async fn from_id(id: impl AsRef<str>) -> Res<Option<Self>> {
        let versions = MinecraftVersionList::fetch().await?;
        Ok(versions.version(id))
    }

    pub async fn metadata(&self) -> Res<MinecraftVersionMetadata> {
        let client = reqwest::ClientBuilder::new()
            .user_agent(format!("{} utils/minecraft-version", USER_AGENT))
            .build()
            .unwrap();
        let response = Error::response_as::<Value>(client.get(&self.url).send().await).await?;

        let client_download: MinecraftFileDownload =
            get_one_at_path("$.downloads.client", &response)?;
        let server_download: MinecraftFileDownload =
            get_one_at_path("$.downloads.server", &response)?;
        let java_version = JavaVersion(get_one_at_path("$.javaVersion.majorVersion", &response)?);

        Ok(MinecraftVersionMetadata {
            client: client_download,
            server: server_download,
            java_version,
            version: self.clone(),
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MinecraftVersionLatest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MinecraftVersionList {
    pub latest: MinecraftVersionLatest,
    pub versions: Vec<MinecraftVersion>,
}

impl MinecraftVersionList {
    pub fn latest_release(&self) -> Option<MinecraftVersion> {
        self.versions
            .iter()
            .find(|v| v.id == self.latest.release)
            .cloned()
    }

    pub fn latest_snapshot(&self) -> Option<MinecraftVersion> {
        self.versions
            .iter()
            .find(|v| v.id == self.latest.snapshot)
            .cloned()
    }

    pub async fn fetch() -> Res<Self> {
        let client = reqwest::ClientBuilder::new()
            .user_agent(format!("{} utils/minecraft-version", USER_AGENT))
            .build()
            .unwrap();
        Error::response_as::<MinecraftVersionList>(client.get(MINECRAFT_VERSIONS_MANIFEST).send().await).await
    }

    pub fn version(&self, id: impl AsRef<str>) -> Option<MinecraftVersion> {
        let id_string = id.as_ref().to_string();
        self.versions.iter().find(|v| v.id == id_string).cloned()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MinecraftFileDownload {
    pub url: String,
    pub sha1: String,
    pub size: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MinecraftVersionMetadata {
    pub client: MinecraftFileDownload,
    pub server: MinecraftFileDownload,
    pub java_version: JavaVersion,
    pub version: MinecraftVersion,
}
