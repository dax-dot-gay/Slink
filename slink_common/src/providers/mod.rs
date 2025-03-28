pub mod error;
pub(in crate::providers) mod server_binary;

pub mod servers {
    pub use super::server_binary::{
        server_binary::{ServerBinaryProvider, ServerBinaryVersion},
        fabric::{FabricServerBinaryProvider, FabricServerBinaryVersion}
    };

    #[derive(Clone, Debug)]
    pub enum Providers {
        Fabric
    }

    impl Providers {
        pub fn get(name: impl AsRef<str>) -> Option<Self> {
            match name.as_ref() {
                "fabric" => Some(Self::Fabric),
                _ => None
            }
        }
    }
}
