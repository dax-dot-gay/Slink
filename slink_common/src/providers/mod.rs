pub mod error;
pub(in crate::providers) mod server_binary;

pub mod servers {
    pub use super::server_binary::{
        server_binary::ServerBinaryProvider,
        fabric_server::{self as fabric, FabricServerBinaryProvider as Fabric}
    };
}