use std::collections::HashMap;

use okapi::openapi3::OpenApi;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use slink_common::{
    ApiError, ApiResult,
    providers::servers::{ServerBinaryProvider, ServerBinaryVersion, Providers as ServerProviders, FabricServerBinaryProvider as Fabric},
    types::MinecraftVersion,
};

use crate::models::User;

#[openapi(tag = "Providers", tag = "Server Binary Provider")]
#[get("/<name>/components")]
async fn get_provider_components(_user: User, name: &str) -> ApiResult<Json<Vec<String>>> {
    match ServerProviders::get(name) {
        Some(ServerProviders::Fabric) => Ok(Fabric::components()),
        None => Err(ApiError::not_found(name)),
    }
    .and_then(|c| Ok(Json(c)))
}

#[openapi(tag = "Providers", tag = "Server Binary Provider")]
#[get("/<name>/<minecraft>/components")]
async fn get_compatible_versions(
    _user: User,
    name: &str,
    minecraft: &str,
) -> ApiResult<Json<HashMap<String, Vec<ServerBinaryVersion>>>> {
    let mcv = match MinecraftVersion::from_id(minecraft).await {
        Ok(Some(version)) => Ok(version),
        Ok(None) => Err(ApiError::not_found(format!(
            "Minecraft version: {minecraft}"
        ))),
        Err(e) => Err(e.into()),
    }?;
    match ServerProviders::get(name) {
        Some(ServerProviders::Fabric) => Fabric::get_components(mcv).await.or_else(|e| Err(e.into())),
        None => Err(ApiError::not_found(name)),
    }
    .and_then(|c| Ok(Json(c)))
}

pub fn routes() -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![
        get_provider_components,
        get_compatible_versions
    ]
}
