use std::{collections::HashMap, fmt::Debug, hash::Hash};

use schemars::JsonSchema;
use serde::{Serialize, de::DeserializeOwned};

use crate::{Error, Res};

use super::super::error::{ProviderError, ProviderType};

#[async_trait::async_trait]
pub trait ServerBinary {
    type ComponentType: Debug
        + Serialize
        + DeserializeOwned
        + Clone
        + JsonSchema
        + Eq
        + PartialEq
        + Hash;
    type ComponentVersion: Debug
        + Serialize
        + DeserializeOwned
        + Clone
        + JsonSchema
        + Eq
        + PartialEq
        + Ord
        + PartialOrd;
    type CompiledVersion: Debug + Serialize + DeserializeOwned + Clone + JsonSchema;

    fn provider_name() -> String;

    fn wrap_error(error: ProviderError) -> Error
    where
        Self: Sized,
    {
        Error::provider_error(ProviderType::ServerBinary, Self::provider_name(), error)
    }

    fn wrap_result<T>(res: Result<T, ProviderError>) -> Res<T>
    where
        Self: Sized,
    {
        res.or_else(|e| Err(Self::wrap_error(e)))
    }

    fn get_component_types() -> Vec<Self::ComponentType>
    where
        Self: Sized;

    fn get_compiled_version(
        components: HashMap<Self::ComponentType, Self::ComponentVersion>,
    ) -> Res<Self::CompiledVersion>
    where
        Self: Sized;

    async fn get_component_versions()
    -> Res<HashMap<Self::ComponentType, Vec<Self::ComponentVersion>>>
    where
        Self: Sized;

    async fn download_server(version: Self::CompiledVersion) -> Res<reqwest::Response>
    where
        Self: Sized;
}
