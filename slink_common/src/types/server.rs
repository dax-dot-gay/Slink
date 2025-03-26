use std::path::Path;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::{Error, Res};

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    #[serde(alias = "0")]
    Peaceful,

    #[serde(alias = "1")]
    Easy,

    #[serde(alias = "2")]
    Normal,

    #[serde(alias = "3")]
    Hard,
}

impl Default for Difficulty {
    fn default() -> Self {
        Self::Easy
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Gamemode {
    #[serde(alias = "0")]
    Survival,

    #[serde(alias = "1")]
    Creative,

    #[serde(alias = "2")]
    Adventure,

    #[serde(alias = "3")]
    Spectator,
}

impl Default for Gamemode {
    fn default() -> Self {
        Self::Survival
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum CompressionType {
    Deflate,
    Lz4,
    None,
}

#[derive(Serialize_repr, Deserialize_repr, Clone, Debug, JsonSchema)]
#[repr(u8)]
pub enum PermissionLevel {
    All = 0,
    Moderator = 1,
    Gamemaster = 2,
    Admin = 3,
    Owner = 4,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "kebab-case", default)]
pub struct ServerProperties {
    pub accepts_transfers: bool,
    pub allow_flight: bool,
    pub allow_nether: bool,
    pub broadcast_console_to_ops: bool,
    pub broadcast_rcon_to_ops: bool,
    pub bug_report_link: Option<String>,
    pub difficulty: Difficulty,
    pub enable_command_block: bool,
    pub enable_jmx_monitoring: bool,
    pub enable_rcon: bool,
    pub enable_status: bool,
    pub enable_query: bool,
    pub enforce_secure_profile: bool,
    pub enforce_whitelist: bool,
    pub entity_broadcast_range_percentage: u16,
    pub force_gamemode: bool,
    pub function_permission_level: PermissionLevel,
    pub gamemode: Gamemode,
    pub generate_structures: bool,
    pub generator_settings: String,
    pub hardcore: bool,
    pub hide_online_players: bool,
    pub initial_disabled_packs: Option<String>,
    pub initial_enabled_packs: Option<String>,
    pub level_name: String,
    pub level_seed: Option<String>,
    pub level_type: String,
    pub log_ips: bool,
    pub max_chained_neighbor_updates: i64,
    pub max_players: u32,
    pub max_tick_time: u64,
    pub max_world_size: u64,
    pub motd: String,
    pub network_compression_threshold: i64,
    pub online_mode: bool,
    pub op_permission_level: PermissionLevel,
    pub pause_when_empty_seconds: i64,
    pub player_idle_timeout: u64,
    pub prevent_proxy_connections: bool,
    pub pvp: bool,

    #[serde(rename = "query.port")]
    pub query_port: u16,
    pub rate_limit: u64,

    #[serde(rename = "rcon.password")]
    pub rcon_password: Option<String>,

    #[serde(rename = "rcon.port")]
    pub rcon_port: u16,
    pub region_file_compression: CompressionType,
    pub resource_pack: Option<String>,
    pub resource_pack_id: Option<String>,
    pub resource_pack_prompt: Option<String>,
    pub resource_pack_sha1: Option<String>,
    pub require_resource_pack: bool,
    pub server_ip: Option<String>,
    pub server_port: u16,
    pub simulation_distance: u8,
    pub spawn_monsters: bool,
    pub spawn_protection: u64,
    pub sync_chunk_writes: bool,
    pub use_native_transport: bool,
    pub view_distance: u8,
    pub white_list: bool,
}

impl Default for ServerProperties {
    fn default() -> Self {
        Self {
            accepts_transfers: false,
            allow_flight: false,
            allow_nether: true,
            broadcast_console_to_ops: true,
            broadcast_rcon_to_ops: true,
            bug_report_link: None,
            difficulty: Difficulty::Easy,
            enable_command_block: false,
            enable_jmx_monitoring: false,
            enable_query: false,
            enable_rcon: false,
            enable_status: true,
            enforce_secure_profile: true,
            enforce_whitelist: false,
            entity_broadcast_range_percentage: 100,
            force_gamemode: false,
            function_permission_level: PermissionLevel::Gamemaster,
            gamemode: Gamemode::Survival,
            generate_structures: true,
            generator_settings: "{}".into(),
            hardcore: false,
            hide_online_players: false,
            initial_disabled_packs: None,
            initial_enabled_packs: Some("vanilla".into()),
            level_name: "world".into(),
            level_seed: None,
            level_type: "minecraft:normal".into(),
            log_ips: true,
            max_chained_neighbor_updates: 1000000,
            max_players: 20,
            max_tick_time: 60000,
            max_world_size: 29999984,
            motd: "A Minecraft Server".into(),
            network_compression_threshold: 256,
            online_mode: true,
            op_permission_level: PermissionLevel::Owner,
            pause_when_empty_seconds: 60,
            player_idle_timeout: 0,
            prevent_proxy_connections: false,
            pvp: true,
            query_port: 25565,
            rate_limit: 0,
            rcon_password: None,
            rcon_port: 25575,
            region_file_compression: CompressionType::Deflate,
            require_resource_pack: false,
            resource_pack: None,
            resource_pack_id: None,
            resource_pack_prompt: None,
            resource_pack_sha1: None,
            server_ip: None,
            server_port: 25565,
            simulation_distance: 10,
            spawn_monsters: true,
            spawn_protection: 16,
            sync_chunk_writes: true,
            use_native_transport: true,
            view_distance: 10,
            white_list: false,
        }
    }
}

impl ServerProperties {
    pub fn from_str(content: impl AsRef<str>) -> Res<Self> {
        serde_java_properties::from_str::<Self>(content.as_ref())
            .or_else(|e| Err(Error::deserialization(e)))
    }

    pub async fn from_file(path: impl AsRef<Path>) -> Res<Self> {
        let mut file = tokio::fs::File::open(path)
            .await
            .or_else(|e| Error::unexpected(e))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .await
            .or_else(|e| Error::unexpected(e))?;
        Self::from_str(contents)
    }

    pub fn to_str(&self) -> Res<String> {
        serde_java_properties::to_string(&self).or_else(|e| Err(Error::serialization(e)))
    }

    pub async fn to_file(&self, path: impl AsRef<Path>) -> Res<()> {
        let serialized = self.to_str()?;
        let mut file = tokio::fs::File::create(path)
            .await
            .or_else(|e| Error::unexpected(e))?;
        file.write_all(serialized.as_bytes())
            .await
            .or_else(|e| Error::unexpected(e))?;
        file.flush().await.or_else(|e| Error::unexpected(e))?;
        Ok(())
    }
}
