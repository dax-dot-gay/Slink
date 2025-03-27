use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{from_value, to_value, Value};

use crate::{Error, Res};

pub mod error;
pub(in crate::providers) mod server_binary;

pub mod servers {
    pub use super::server_binary::{
        server_binary::ServerBinaryProvider,
        fabric_server::{self as fabric, FabricServerBinaryProvider as Fabric}
    };

    #[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
    pub enum VersionTypes {
        Fabric(servers::fabric::FabricServerBinaryVersion)
    }
}
