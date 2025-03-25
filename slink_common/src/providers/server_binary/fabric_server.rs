use std::collections::HashMap;

use reqwest::{Client, ClientBuilder};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Res, USER_AGENT};

use super::server_binary::ServerBinaryProvider;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, Eq, PartialEq)]
#[serde(tag = "component", rename_all = "snake_case")]
pub enum FabricServerBinaryVersion {
    Loader {
        version: String,
        stable: bool
    },
    Installer {
        version: String,
        stable: bool
    }
}

impl FabricServerBinaryVersion {
    pub fn version(&self) -> String {
        match self {
            Self::Loader { version, .. } => version.clone(),
            Self::Installer { version, .. } => version.clone()
        }
    }

    pub fn kind(&self) -> String {
        match self {
            Self::Loader { ..} => "loader",
            Self::Installer { .. } => "installer"
        }.to_string()
    }
}

pub struct FabricServerBinaryProvider;

impl ServerBinaryProvider for FabricServerBinaryProvider {
    
}

impl FabricServerBinaryProvider {
    fn client() -> Client {
        ClientBuilder::new().user_agent(format!("{} {}", USER_AGENT, FabricServerBinaryProvider::id())).build().unwrap()
    }
}
