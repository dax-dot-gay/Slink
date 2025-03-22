use std::{error::Error, path::PathBuf, str::FromStr};
use futures::StreamExt;

use bytesize::ByteSize;
use slink_common::{runners::{docker_host::{DockerHostRunner, DockerHostRunnerOptions}, MinecraftRunner, MinecraftRunnerConfig, MinecraftRunnerPort, PortExposure}, types::minecraft::JavaVersion};
use uuid::Uuid;

const SERVER_ID: &'static str = "ec9612a8-d60d-427f-8857-331c62af0ea8";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let id = Uuid::from_str(SERVER_ID).unwrap();
    let config = MinecraftRunnerConfig {
        runner_id: id,
        java_version: JavaVersion(21),
        max_memory: ByteSize::gb(8),
        binary: String::from("server.jar"),
        java_args: vec![String::from("-Xmx4G")],
        minecraft_args: vec![String::from("--nogui")],
        ports: vec![
            MinecraftRunnerPort::Server(25565, 25565, PortExposure::Host),
            MinecraftRunnerPort::Rcon(25575, 25575, PortExposure::Host),
            MinecraftRunnerPort::Query(25565, 25565, PortExposure::Host)
        ]
    };
    let options = DockerHostRunnerOptions {
        network: String::from("bridge"),
        host_base_path: PathBuf::from_str("/mnt/data1/Programming/Rust/Slink/volumes/servers").unwrap(),
        run_as: String::from("1000:1000")
    };

    let mut runner = DockerHostRunner::new(config, options)?;
    runner.install().await?;
    runner.start().await?;

    let mut reader = runner.get_reader().await?;
    while let Some(Some(item)) = reader.next().await {
        println!("{item:?}");
    }

    Ok(())
}
