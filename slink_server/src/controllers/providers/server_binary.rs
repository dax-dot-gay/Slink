use std::collections::HashMap;

use rocket::serde::json::Json;
use rocket_okapi::openapi;
use slink_common::{
    ApiError, ApiResult,
    providers::{
        ServerBinaryVersionTypes,
        servers::{Fabric, Providers as ServerProviders, ServerBinaryProvider as _},
    },
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
async fn get_provider_components(
    _user: User,
    name: &str,
    minecraft: &str,
) -> ApiResult<Json<HashMap<String, Vec<ServerBinaryVersionTypes>>>> {
    let mcv = match MinecraftVersion::from_id(minecraft).await {
        Ok(Some(version)) => Ok(version),
        Ok(None) => Err(ApiError::not_found(format!(
            "Minecraft version: {minecraft}"
        ))),
        Err(e) => Err(e.into()),
    }?;
    match ServerProviders::get(name) {
        Some(ServerProviders::Fabric) => Fabric::get_components(mcv).await.and_then(|c| ),
        None => Err(ApiError::not_found(name)),
    }
    .and_then(|c| Ok(Json(c)))
}
