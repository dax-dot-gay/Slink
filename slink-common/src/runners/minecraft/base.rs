use std::{fmt::Debug, path::PathBuf};

use bytesize::ByteSize;
use futures::{AsyncRead, AsyncWrite};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use uuid::Uuid;

use crate::{
    error::{Error, Res},
    types::minecraft::JavaVersion,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MinecraftRunnerPort {
    Server(u16),
    Rcon(u16),
    Query(u16),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MinecraftRunnerConfig {
    pub runner_id: Uuid,
    pub java_version: JavaVersion,
    pub max_memory: ByteSize,
    pub slink_dir: PathBuf,
    pub serve_dir: PathBuf,
    pub binary: String,
    pub additional_arguments: Vec<String>,
    pub ports: Vec<MinecraftRunnerPort>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MinecraftRunnerStatus {
    Running,
    Starting,
    Stopping,
    Restarting,
    OfflineStopped(Option<String>),
    OfflineFailed(Option<Error>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MinecraftRunnerMetrics {
    pub memory: Option<ByteSize>,
    pub max_memory: Option<ByteSize>,
    pub cpu_usage: Option<u64>,
}

#[async_trait::async_trait]
pub trait MinecraftRunner: Clone {
    type Options: Serialize + DeserializeOwned + Clone + Debug;

    fn new(config: MinecraftRunnerConfig, options: Self::Options)
    where
        Self: Sized;

    fn config(&self) -> MinecraftRunnerConfig;
    async fn status(&self) -> MinecraftRunnerStatus;
    async fn start(&mut self) -> Res<MinecraftRunnerStatus>;
    async fn stop(&mut self) -> Res<MinecraftRunnerStatus>;
    async fn metrics(&self) -> Res<MinecraftRunnerMetrics>;
    async fn get_reader(&self) -> Res<Box<dyn AsyncRead>>;
    async fn get_writer(&self) -> Res<Box<dyn AsyncWrite>>;
}
