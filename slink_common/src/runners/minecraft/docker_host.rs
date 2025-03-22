use std::{collections::HashMap, path::PathBuf, pin::Pin};

use bollard::{container, image, secret};
use bytes::Bytes;
use bytesize::ByteSize;
use futures::{Stream, StreamExt as _};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWrite;

use crate::{CONTAINER_WORKING_DIRECTORY, error::Res};

use super::base::{
    MinecraftRunner, MinecraftRunnerConfig, MinecraftRunnerMetrics, MinecraftRunnerStatus,
    PortExposure,
};

#[derive(thiserror::Error, Clone, Debug, Serialize, Deserialize)]
pub enum DockerHostError {
    #[error("Failed to connect to the host's Docker daemon: {0}")]
    ConnectionError(String),

    #[error("Failed to create container: {0}")]
    ContainerCreationError(String),

    #[error("Invalid operation given the current context.")]
    InvalidOp,

    #[error("Encountered an error with an underlying Docker operation: {0}")]
    DockerError(String),

    #[error("Encountered an error while retrieving the status of a container: {0}")]
    StatusError(String),

    #[error("Unknown/invalid container image ({0}): {1}")]
    BadImage(String, String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DockerHostRunnerOptions {
    pub network: String,
    pub host_base_path: PathBuf,
    pub run_as: String
}

#[derive(Clone)]
pub struct DockerHostRunner {
    connection: bollard::Docker,
    config: MinecraftRunnerConfig,
    options: DockerHostRunnerOptions,
    status: MinecraftRunnerStatus,
    container_id: Option<String>,
}

impl DockerHostRunner {
    pub fn container_name(&self) -> String {
        format!(
            "slink.mc-server.docker-host.{}",
            self.config.runner_id.to_string()
        )
    }

    pub fn container_id(&self) -> Option<String> {
        self.container_id.clone()
    }
}

#[async_trait::async_trait]
impl MinecraftRunner for DockerHostRunner {
    type Options = DockerHostRunnerOptions;
    type Error = DockerHostError;

    fn new(config: MinecraftRunnerConfig, options: Self::Options) -> Res<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            connection: bollard::Docker::connect_with_local_defaults().or_else(|e| {
                Err(Self::wrap_anon(
                    config.runner_id,
                    DockerHostError::ConnectionError(e.to_string()),
                ))
            })?,
            config,
            options,
            status: MinecraftRunnerStatus::Uninitialized,
            container_id: None,
        })
    }

    fn runner_type() -> String {
        String::from("docker-host")
    }

    fn config(&self) -> MinecraftRunnerConfig {
        self.config.clone()
    }

    fn options(&self) -> Self::Options {
        self.options.clone()
    }

    async fn install(&mut self) -> Res<()> {
        if let Err(_) = self
            .connection
            .inspect_image(&self.config.java_version.image())
            .await
        {
            self.connection
                .create_image(
                    Some(image::CreateImageOptions {
                        from_image: self.config.java_version.image(),
                        ..Default::default()
                    }),
                    None,
                    None,
                )
                .next()
                .await
                .ok_or(self.wrap(DockerHostError::BadImage(
                    self.config.java_version.image(),
                    String::from("Not found."),
                )))?
                .or_else(|e| Err(self.wrap(DockerHostError::DockerError(e.to_string()))))?;
        }

        let _ = self.connection.remove_container(&self.container_name(), Some(container::RemoveContainerOptions {force: true, ..Default::default()})).await;

        let mut ports: HashMap<String, HashMap<(), ()>> = HashMap::new();
        let mut port_mappings: HashMap<String, Option<Vec<secret::PortBinding>>> = HashMap::new();
        for port in self.config.ports.clone() {
            ports.insert(
                format!("{}/{}", port.local(), port.protocol()),
                HashMap::new(),
            );

            match port.exposure() {
                PortExposure::Runner => {}
                PortExposure::Host => {
                    port_mappings.insert(
                        format!("{}/{}", port.local(), port.protocol()),
                        Some(vec![secret::PortBinding {
                            host_ip: Some(String::from("127.0.0.1")),
                            host_port: Some(port.exposed().to_string()),
                        }]),
                    );
                }
                PortExposure::Global => {
                    port_mappings.insert(
                        format!("{}/{}", port.local(), port.protocol()),
                        Some(vec![secret::PortBinding {
                            host_ip: None,
                            host_port: Some(port.exposed().to_string()),
                        }]),
                    );
                }
            }
        }

        let options = Some(container::CreateContainerOptions {
            name: self.container_name(),
            platform: None,
        });

        let mut cmd: Vec<String> = vec![String::from("java")];
        cmd.extend(self.config.java_args.clone());
        cmd.push(String::from("-jar"));
        cmd.push(self.config.binary.clone());
        cmd.extend(self.config.minecraft_args.clone());

        let config = container::Config {
            hostname: Some(self.container_name()),
            user: Some(self.options.run_as.clone()),
            exposed_ports: Some(ports),
            image: Some(self.config.java_version.image()),
            cmd: Some(cmd),
            working_dir: Some(CONTAINER_WORKING_DIRECTORY.to_string()),
            host_config: Some(secret::HostConfig {
                memory: Some(self.config.max_memory.as_u64() as i64),
                binds: Some(vec![format!(
                    "{}:{}",
                    self.options
                        .host_base_path
                        .join(self.id())
                        .to_str()
                        .unwrap(),
                    CONTAINER_WORKING_DIRECTORY
                )]),
                network_mode: Some(self.options.network.clone()),
                port_bindings: Some(port_mappings),
                ..Default::default()
            }),
            ..Default::default()
        };

        let result = match self.connection.create_container(options, config).await {
            Ok(r) => r,
            Err(e) => {
                self.container_id = None;
                self.status = MinecraftRunnerStatus::Failed(
                    self.wrap(DockerHostError::ContainerCreationError(e.to_string())),
                );
                return Err(self.wrap(DockerHostError::ContainerCreationError(e.to_string())));
            }
        };

        self.container_id = Some(result.id.clone());
        self.status = MinecraftRunnerStatus::Offline(None);

        Ok(())
    }

    async fn uninstall(&mut self) -> Res<()> {
        if !self.status.initialized() {
            return Err(self.wrap(DockerHostError::InvalidOp));
        }

        self.connection
            .remove_container(
                &self.container_name(),
                Some(container::RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                }),
            )
            .await
            .or_else(|e| Err(self.wrap(DockerHostError::DockerError(e.to_string()))))?;
        self.container_id = None;
        self.status = MinecraftRunnerStatus::Uninitialized;
        Ok(())
    }

    async fn start(&mut self) -> Res<MinecraftRunnerStatus> {
        if !self.status.initialized() || self.status.running() {
            return Err(self.wrap(DockerHostError::InvalidOp));
        }

        self.connection
            .start_container(
                &self.container_name(),
                None::<container::StartContainerOptions<String>>,
            )
            .await
            .or_else(|e| Err(self.wrap(DockerHostError::DockerError(e.to_string()))))?;

        self.status = MinecraftRunnerStatus::Running;
        Ok(MinecraftRunnerStatus::Running)
    }

    async fn stop(&mut self) -> Res<MinecraftRunnerStatus> {
        if !self.status.running() {
            return Err(self.wrap(DockerHostError::InvalidOp));
        }

        self.connection
            .stop_container(&self.container_name(), None)
            .await
            .or_else(|e| Err(self.wrap(DockerHostError::DockerError(e.to_string()))))?;

        self.status = MinecraftRunnerStatus::Offline(None);
        Ok(MinecraftRunnerStatus::Offline(None))
    }

    async fn status(&mut self) -> MinecraftRunnerStatus {
        let new_status = match self.status.clone() {
            MinecraftRunnerStatus::Running => {
                match self
                    .connection
                    .inspect_container(&self.container_name(), None)
                    .await
                {
                    Ok(inspection) => {
                        if let Some(state) = inspection.state {
                            if let Some(secret::ContainerStateStatusEnum::RUNNING) = state.status {
                                MinecraftRunnerStatus::Running
                            } else if let Some(err) = state.error {
                                MinecraftRunnerStatus::Offline(Some(
                                    self.wrap(DockerHostError::DockerError(err)),
                                ))
                            } else {
                                MinecraftRunnerStatus::Offline(None)
                            }
                        } else {
                            MinecraftRunnerStatus::Offline(Some(self.wrap(
                                DockerHostError::StatusError(String::from(
                                    "Container state is inaccessible.",
                                )),
                            )))
                        }
                    }
                    Err(_) => MinecraftRunnerStatus::Failed(self.wrap(
                        DockerHostError::StatusError(String::from(
                            "Container is entirely inaccessible. Switching to FAILED.",
                        )),
                    )),
                }
            }
            status => status,
        };

        self.status = new_status.clone();
        new_status
    }

    async fn metrics(&self) -> Res<Box<dyn Stream<Item = Option<MinecraftRunnerMetrics>> + Send>> {
        if !self.status.running() {
            return Err(self.wrap(DockerHostError::InvalidOp));
        }

        let stats = self.connection.stats(&self.container_name(), None);
        Ok(Box::new(stats.map(|item| match item {
            Ok(data) => Some(MinecraftRunnerMetrics {
                memory: data.memory_stats.usage.and_then(|u| Some(ByteSize::b(u))),
                max_memory: data.memory_stats.limit.and_then(|u| Some(ByteSize::b(u))),
                cpu_usage: data.cpu_stats.system_cpu_usage,
            }),
            Err(_) => None,
        })))
    }

    async fn get_reader(&self) -> Res<Pin<Box<dyn Stream<Item = Option<Bytes>> + Send>>> {
        if !self.status.running() {
            return Err(self.wrap(DockerHostError::InvalidOp));
        }

        let attach = self
            .connection
            .attach_container(
                &self.container_name(),
                Some(container::AttachContainerOptions {
                    stdin: Some(true),
                    stdout: Some(true),
                    stderr: Some(true),
                    logs: Some(false),
                    stream: Some(true),
                    detach_keys: Some("ctrl-c".to_string()),
                }),
            )
            .await
            .or_else(|e| Err(self.wrap(DockerHostError::DockerError(e.to_string()))))?;

        Ok(Box::pin(attach.output.map(|i| {
            if let Ok(log) = i {
                Some(log.into_bytes())
            } else {
                None
            }
        })))
    }

    async fn get_writer(&self) -> Res<Box<dyn AsyncWrite + Send>> {
        if !self.status.running() {
            return Err(self.wrap(DockerHostError::InvalidOp));
        }

        let attach = self
            .connection
            .attach_container(
                &self.container_name(),
                Some(container::AttachContainerOptions {
                    stdin: Some(true),
                    stdout: Some(true),
                    stderr: Some(true),
                    logs: Some(false),
                    stream: Some(true),
                    detach_keys: Some("ctrl-c".to_string()),
                }),
            )
            .await
            .or_else(|e| Err(self.wrap(DockerHostError::DockerError(e.to_string()))))?;
        Ok(Box::new(attach.input))
    }
}
