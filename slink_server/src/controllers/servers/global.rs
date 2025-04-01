use bson::doc;
use futures::TryStreamExt;
use manor::Model;
use okapi::openapi3::OpenApi;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use slink_common::{providers::servers::ServerBinaryVersion, types::MinecraftVersion, ApiError, ApiResult, Error};

use crate::{models::{MinecraftServer, MinecraftServerBuilder, User}, util::Docs};

#[openapi(tag = "Servers", tag = "GlobalServers")]
#[get("/owned")]
async fn get_owned_servers(user: User, servers: Docs<MinecraftServer>) -> ApiResult<Json<Vec<MinecraftServer>>> {
    let cursor = servers.find_many(doc! {"owner.id": user.id}).await.or_else(|e| Err(ApiError::from(Error::Unexpected(e.to_string()))))?;
    let results = cursor.try_collect::<Vec<MinecraftServer>>().await.or_else(|e| Err(ApiError::from(Error::Unexpected(e.to_string()))))?;

    Ok(Json(results))
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
struct ServerCreationParams {
    pub name: String,
    pub minecraft_version: String,

    #[serde(default)]
    pub mod_loader: Option<ServerBinaryVersion>
}

#[openapi(tag = "Servers", tag = "GlobalServers")]
#[post("/create", data = "<create>")]
async fn create_server(user: User, create: Json<ServerCreationParams>) -> ApiResult<Json<MinecraftServer>> {
    let params = create.into_inner();
    let minecraft_version = match MinecraftVersion::from_id(params.minecraft_version.clone()).await {
        Ok(Some(version)) => match version.metadata().await {
            Ok(metadata) => metadata,
            Err(e) => {return Err(ApiError::from(e));}
        },
        Ok(None) => {return Err(ApiError::NotFound(params.minecraft_version.clone()));},
        Err(e) => {return Err(ApiError::from(e));}
    };

    let mut server_builder = &mut MinecraftServerBuilder::default();
    server_builder = server_builder.name(params.name.clone()).minecraft_version(minecraft_version).owner(user.clone());
    if let Some(modloader) = params.mod_loader.clone() {
        server_builder = server_builder.modloader_version(modloader);
    }
    let new_server = server_builder.build().or_else(|e| Err(ApiError::from(Error::Unexpected(e.to_string()))))?;
    
    new_server.save().await.or_else(|e| Err(ApiError::from(Error::Unexpected(e.to_string()))))?;

    Ok(Json(new_server))
}

pub fn routes() -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![
        get_owned_servers,
        create_server
    ]
}
