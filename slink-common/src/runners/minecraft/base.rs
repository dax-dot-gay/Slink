use std::fmt::Debug;

use bytes::Bytes;
use bytesize::ByteSize;
use futures::Stream;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tokio::io::AsyncWrite;
use uuid::Uuid;

use crate::{
    error::{Error, Res},
    types::minecraft::JavaVersion,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PortExposure {
    /// Expose only within the runner's network (same as Host if not running in a container)
    Runner,

    /// Expose to the runner's host, but not outside that system.
    Host,

    /// Fully expose ports
    Global
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MinecraftRunnerPort {
    Server(u16, u16, PortExposure),
    Rcon(u16, u16, PortExposure),
    Query(u16, u16, PortExposure),
}

impl MinecraftRunnerPort {
    pub fn local(&self) -> u16 {
        match self {
            Self::Server(p, ..) => p.clone(),
            Self::Rcon(p, ..) => p.clone(),
            Self::Query(p, ..) => p.clone(),
        }
    }

    pub fn exposed(&self) -> u16 {
        match self {
            Self::Server(_, p, _) => p.clone(),
            Self::Rcon(_, p, _) => p.clone(),
            Self::Query(_, p, _) => p.clone(),
        }
    }

    pub fn protocol(&self) -> String {
        match self {
            Self::Server(..) => String::from("tcp"),
            Self::Rcon(..) => String::from("tcp"),
            Self::Query(..) => String::from("udp"),
        }
    }

    pub fn exposure(&self) -> PortExposure {
        match self {
            Self::Server(_, _, e) => e.clone(),
            Self::Rcon(_, _, e) => e.clone(),
            Self::Query(_, _, e) => e.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MinecraftRunnerConfig {
    pub runner_id: Uuid,
    pub java_version: JavaVersion,
    pub max_memory: ByteSize,
    pub binary: String,
    pub java_args: Vec<String>,
    pub minecraft_args: Vec<String>,
    pub ports: Vec<MinecraftRunnerPort>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MinecraftRunnerStatus {
    Uninitialized,
    Running,
    Offline(Option<Error>),
    Failed(Error)
}

impl MinecraftRunnerStatus {
    pub fn initialized(&self) -> bool {
        match self {
            Self::Uninitialized | Self::Failed(_) => false,
            _ => true
        }
    }

    pub fn error(&self) -> Option<Error> {
        match self {
            Self::Offline(err) => err.clone(),
            Self::Failed(err) => Some(err.clone()),
            _ => None
        }
    }

    pub fn running(&self) -> bool {
        if let Self::Running = self {
            true
        } else {
            false
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MinecraftRunnerMetrics {
    pub memory: Option<ByteSize>,
    pub max_memory: Option<ByteSize>,
    pub cpu_usage: Option<u64>
}

#[async_trait::async_trait]
pub trait MinecraftRunner: Clone {
    type Options: Serialize + DeserializeOwned + Clone + Debug;
    type Error: std::error::Error;

    fn new(config: MinecraftRunnerConfig, options: Self::Options) -> Res<Self> where Self: Sized;
    fn wrap(&self, error: Self::Error) -> Error {
        Error::RunnerError { scope: String::from("minecraft"), runner: Self::runner_type(), id: self.config().runner_id.to_string(), reason: error.to_string() }
    }

    fn wrap_anon(id: Uuid, error: Self::Error) -> Error {
        Error::RunnerError { scope: String::from("minecraft"), runner: Self::runner_type(), id: id.to_string(), reason: error.to_string() }
    }

    fn id(&self) -> String {
        self.config().runner_id.to_string()
    }

    fn runner_type() -> String;
    fn config(&self) -> MinecraftRunnerConfig;
    fn options(&self) -> Self::Options;
    async fn install(&mut self) -> Res<()>;
    async fn uninstall(&mut self) -> Res<()>;
    async fn status(&mut self) -> MinecraftRunnerStatus;
    async fn start(&mut self) -> Res<MinecraftRunnerStatus>;
    async fn stop(&mut self) -> Res<MinecraftRunnerStatus>;
    async fn metrics(&self) -> Res<Box<dyn Stream<Item = Option<MinecraftRunnerMetrics>> + Send>>;
    async fn get_reader(&self) -> Res<Box<dyn Stream<Item = Option<Bytes>> + Send>>;
    async fn get_writer(&self) -> Res<Box<dyn AsyncWrite + Send>>;
}
