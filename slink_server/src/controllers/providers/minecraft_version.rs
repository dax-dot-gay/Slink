use okapi::openapi3::OpenApi;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use slink_common::{
    types::{MinecraftVersion, MinecraftVersionList, MinecraftVersionMetadata}, ApiError, ApiResult
};
use slink_macros::cache;

use crate::models::User;

#[cache(key = "providers.mc_version.versions", life_time = "15m")]
#[openapi(tag = "Providers", tag = "Minecraft Version Provider")]
#[get("/versions")]
async fn list_minecraft_versions(_user: User) -> ApiResult<Json<MinecraftVersionList>> {
    MinecraftVersionList::fetch()
        .await
        .or_else(|e| Err(e.into()))
        .and_then(|v| Ok(Json(v)))
}

#[cache(key = "providers.mc_version.latest.release", life_time = "15m")]
#[openapi(tag = "Providers", tag = "Minecraft Version Provider")]
#[get("/versions/latest_release")]
async fn get_latest_release_version(_user: User) -> ApiResult<Json<MinecraftVersion>> {
    let versions = MinecraftVersionList::fetch()
        .await
        .or_else(|e| Err::<_, ApiError>(e.into()))?;
    if let Some(latest) = versions.latest_release() {
        Ok(Json(latest))
    } else {
        Err(ApiError::not_found("<LATEST RELEASE>"))
    }
}

#[cache(key = "providers.mc_version.latest.snapshot", life_time = "15m")]
#[openapi(tag = "Providers", tag = "Minecraft Version Provider")]
#[get("/versions/latest_snapshot")]
async fn get_latest_snapshot_version(_user: User) -> ApiResult<Json<MinecraftVersion>> {
    let versions = MinecraftVersionList::fetch()
        .await
        .or_else(|e| Err::<_, ApiError>(e.into()))?;
    if let Some(latest) = versions.latest_snapshot() {
        Ok(Json(latest))
    } else {
        Err(ApiError::not_found("<LATEST SNAPSHOT>"))
    }
}

#[cache(key = "providers.mc_version.{id}", life_time = "15m")]
#[openapi(tag = "Providers", tag = "Minecraft Version Provider")]
#[get("/versions/<id>")]
async fn get_specific_version(_user: User, id: &str) -> ApiResult<Json<MinecraftVersion>> {
    let versions = MinecraftVersionList::fetch()
        .await
        .or_else(|e| Err::<_, ApiError>(e.into()))?;
    if let Some(selected) = versions.version(id) {
        Ok(Json(selected))
    } else {
        Err(ApiError::not_found(id))
    }
}

#[cache(key = "providers.mc_version.{id}.metadata", life_time = "15m")]
#[openapi(tag = "Providers", tag = "Minecraft Version Provider")]
#[get("/versions/<id>/metadata")]
async fn get_version_metadata(_user: User, id: &str) -> ApiResult<Json<MinecraftVersionMetadata>> {
    let versions = MinecraftVersionList::fetch()
        .await
        .or_else(|e| Err::<_, ApiError>(e.into()))?;
    if let Some(selected) = versions.version(id) {
        selected.metadata().await.and_then(|m| Ok(Json(m))).or_else(|e|  Err(e.into()))
    } else {
        Err(ApiError::not_found(id))
    }
}

pub fn routes() -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![
        list_minecraft_versions,
        get_latest_release_version,
        get_latest_snapshot_version,
        get_specific_version,
        get_version_metadata
    ]
}
