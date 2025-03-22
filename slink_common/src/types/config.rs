use std::path::PathBuf;

use mongodb::options::ClientOptions;
use serde::Deserialize;

use crate::runners::docker_host::DockerHostRunnerOptions;

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum DatabaseConfig {
    Uri {
        uri: String,
        database: String,
    },
    Options {
        options: ClientOptions,
        database: String,
    },
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum RunnerConfig {
    DockerHost {
        network: String,
        host_base_path: PathBuf,
        user: u32,
        group: u32,
    },
}

impl RunnerConfig {
    pub fn as_docker_host(&self) -> Option<DockerHostRunnerOptions> {
        #[allow(irrefutable_let_patterns)]
        if let Self::DockerHost {
            network,
            host_base_path,
            user,
            group,
        } = self
        {
            Some(DockerHostRunnerOptions {
                network: network.clone(),
                host_base_path: host_base_path.clone(),
                run_as: format!("{user}:{group}"),
            })
        } else {
            None
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub runner: RunnerConfig,
}
