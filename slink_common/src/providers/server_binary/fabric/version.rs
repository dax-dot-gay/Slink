use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, Eq, PartialEq)]
#[serde(tag = "component", rename_all = "snake_case")]
pub enum FabricServerBinaryVersion {
    Loader { version: String, stable: bool },
    Installer { version: String, stable: bool },
}

impl FabricServerBinaryVersion {
    pub fn version(&self) -> String {
        match self {
            Self::Loader { version, .. } => version.clone(),
            Self::Installer { version, .. } => version.clone(),
        }
    }

    pub fn kind(&self) -> String {
        match self {
            Self::Loader { .. } => "loader",
            Self::Installer { .. } => "installer",
        }
        .to_string()
    }

    pub fn stable(&self) -> bool {
        match self {
            Self::Loader { stable, .. } => *stable,
            Self::Installer { stable, .. } => *stable,
        }
    }
}
