use std::{collections::HashMap, fmt::Debug, path::Path};

use schemars::JsonSchema;
use serde::{Serialize, de::DeserializeOwned};

use crate::{types::minecraft::MinecraftVersion, Error, Res};

use super::super::error::{ProviderError, ProviderType};

#[async_trait::async_trait]
pub trait ServerBinaryProvider {
    type VersionComponent: Serialize + DeserializeOwned + Clone + Debug + JsonSchema + Eq + PartialEq;

    fn components() -> Vec<String> where Self: Sized;
    fn name() -> String where Self: Sized;
    async fn get_components(minecraft_version: MinecraftVersion) -> Res<HashMap<String, Vec<Self::VersionComponent>>> where Self: Sized;
    async fn install_to(minecraft_version: MinecraftVersion, components: HashMap<String, Self::VersionComponent>, directory: impl AsRef<Path>) -> Res<()> where Self: Sized;

    fn result<T: Sized>(res: Result<T, ProviderError>) -> Res<T> where Self: Sized {
        res.or_else(|e| Err(Error::provider_error(ProviderType::ServerBinary, Self::name(), e)))
    }

    fn error(err: ProviderError) -> Error where Self: Sized{
        Error::provider_error(ProviderType::ServerBinary, Self::name(), err)
    }

    fn id() -> String where Self: Sized {
        format!("providers/server_binary/{}", Self::name())
    }
}
