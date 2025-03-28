use std::{collections::HashMap, path::PathBuf};

use futures::StreamExt;
use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::io::AsyncWriteExt;

use crate::{
    Res, SERVER_BINARY_NAME, USER_AGENT, providers::error::ProviderError,
    types::minecraft::MinecraftVersion, utilities::get_at_path,
};

use super::{FabricServerBinaryVersion, ServerBinaryProvider, ServerBinaryVersion};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct LoaderVersion {
    pub version: String,
    pub stable: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct InstallerVersion {
    pub version: String,
    pub stable: bool,
}

pub struct FabricServerBinaryProvider;

impl FabricServerBinaryProvider {
    #[allow(irrefutable_let_patterns)]
    fn as_fabric_version(version: ServerBinaryVersion) -> Res<FabricServerBinaryVersion> {
        if let ServerBinaryVersion::Fabric(v) = version {
            Ok(v)
        } else {
            Err(Self::error(ProviderError::IncorrectArg(String::from(
                "ServerBinaryVersion::Fabric",
            ))))
        }
    }

    fn as_global_version(version: FabricServerBinaryVersion) -> ServerBinaryVersion {
        ServerBinaryVersion::Fabric(version)
    }
}

#[async_trait::async_trait]
impl ServerBinaryProvider for FabricServerBinaryProvider {
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
    ) -> Res<HashMap<String, Vec<ServerBinaryVersion>>>
    where
        Self: Sized,
    {
        let loader_versions = get_at_path::<LoaderVersion>(
            "$[*].loader",
            &Self::result(
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
            )?,
        )?;
        let installer_versions = Self::result(
            ProviderError::response_as::<Vec<InstallerVersion>>(
                Self::client()
                    .get("https://meta.fabricmc.net/v2/versions/installer")
                    .send()
                    .await,
            )
            .await,
        )?;

        let mut components: HashMap<String, Vec<ServerBinaryVersion>> = HashMap::new();
        components.insert(
            String::from("loader"),
            loader_versions
                .iter()
                .map(|v| {
                    ServerBinaryVersion::Fabric(FabricServerBinaryVersion::Loader {
                        version: v.version.clone(),
                        stable: v.stable,
                    })
                })
                .collect(),
        );
        components.insert(
            String::from("installer"),
            installer_versions
                .iter()
                .map(|v| {
                    ServerBinaryVersion::Fabric(FabricServerBinaryVersion::Installer {
                        version: v.version.clone(),
                        stable: v.stable,
                    })
                })
                .collect(),
        );

        Ok(components)
    }

    async fn install_to(
        minecraft_version: MinecraftVersion,
        components: HashMap<String, ServerBinaryVersion>,
        directory: PathBuf,
    ) -> Res<()>
    where
        Self: Sized,
    {
        let loader_version = Self::as_fabric_version(
            Self::result(components.get(&String::from("loader")).ok_or(
                ProviderError::MissingVersionComponent(String::from("loader")),
            ))?
            .clone(),
        )?;
        let installer_version = Self::as_fabric_version(
            Self::result(components.get(&String::from("installer")).ok_or(
                ProviderError::MissingVersionComponent(String::from("installer")),
            ))?
            .clone(),
        )?;

        let response = Self::result(ProviderError::response(Self::client().get(format!("https://meta.fabricmc.net/v2/versions/loader/{mc}/{loader}/{installer}/server/jar", mc=minecraft_version.id, loader=loader_version.version(), installer=installer_version.version())).send().await))?;
        if let Ok(mut file) = tokio::fs::File::create_new(directory.join(SERVER_BINARY_NAME)).await
        {
            let mut stream = response.bytes_stream();
            while let Some(chunk_result) = stream.next().await {
                if let Ok(chunk) = chunk_result {
                    Self::result(file.write_all(&chunk).await.or_else(|e| {
                        Err(ProviderError::DownloadError {
                            path: directory
                                .join(SERVER_BINARY_NAME)
                                .to_str()
                                .unwrap_or("BAD_PATH")
                                .to_string(),
                            reason: e.to_string(),
                        })
                    }))?;
                } else {
                    return Err(Self::error(ProviderError::DownloadError {
                        path: directory
                            .join(SERVER_BINARY_NAME)
                            .to_str()
                            .unwrap_or("BAD_PATH")
                            .to_string(),
                        reason: String::from("Failed to read chunk from network."),
                    }));
                }
            }

            let _ = file.flush().await;
            Ok(())
        } else {
            Err(Self::error(ProviderError::DownloadError {
                path: directory
                    .join(SERVER_BINARY_NAME)
                    .to_str()
                    .unwrap_or("BAD_PATH")
                    .to_string(),
                reason: String::from("Failed to create file."),
            }))
        }
    }

    async fn get_latest_stable_component(
        minecraft_version: MinecraftVersion,
        component: &str,
    ) -> Res<ServerBinaryVersion>
    where
        Self: Sized,
    {
        let components = Self::get_components(minecraft_version.clone()).await?;
        if let Some(versions) = components.get(&component.to_string()) {
            if let Some(latest) = versions
                .iter()
                .filter_map(|v| {
                    if let Ok(vers) = Self::as_fabric_version(v.clone()) {
                        if vers.stable() {
                            Some(Self::as_global_version(vers.clone()))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<ServerBinaryVersion>>()
                .first()
            {
                Ok(latest.clone())
            } else {
                Err(Self::error(ProviderError::NoVersions {
                    component: component.to_string(),
                    mc_version: minecraft_version.id,
                }))
            }
        } else {
            Err(Self::error(ProviderError::UnknownVersionComponent(
                component.to_string(),
            )))
        }
    }

    async fn get_latest_unstable_component(
        minecraft_version: MinecraftVersion,
        component: &str,
    ) -> Res<ServerBinaryVersion>
    where
        Self: Sized,
    {
        let components = Self::get_components(minecraft_version.clone()).await?;
        if let Some(versions) = components.get(&component.to_string()) {
            if let Some(latest) = versions
                .iter()
                .filter_map(|v| {
                    if let Ok(vers) = Self::as_fabric_version(v.clone()) {
                        if !vers.stable() {
                            Some(Self::as_global_version(vers.clone()))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<ServerBinaryVersion>>()
                .first()
            {
                Ok(latest.clone())
            } else {
                Err(Self::error(ProviderError::NoVersions {
                    component: component.to_string(),
                    mc_version: minecraft_version.id,
                }))
            }
        } else {
            Err(Self::error(ProviderError::UnknownVersionComponent(
                component.to_string(),
            )))
        }
    }
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
