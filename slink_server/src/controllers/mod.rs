use rocket::{serde::json::Json, Rocket};
use rocket_okapi::{
    mount_endpoints_and_merged_docs, openapi, openapi_get_routes_spec, rapidoc::{make_rapidoc, GeneralConfig}, settings::{OpenApiSettings, UrlObject}
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use slink_common::types::{AppConfig, RunnerMode};

use crate::models::Session;

pub mod authentication;

#[derive(JsonSchema, Serialize, Deserialize, Clone, Debug)]
pub struct IndexInfo {
    pub session: Session,
    pub runner_mode: RunnerMode
}

#[openapi]
#[get("/")]
async fn get_index(session: Session, config: AppConfig) -> Json<IndexInfo> {
    Json(IndexInfo {
        session: session.clone(),
        runner_mode: config.runner.mode()
    })
}

pub fn apply(mut rocket: Rocket<rocket::Build>) -> Rocket<rocket::Build> {
    let settings = OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
        rocket, "/".to_owned(), settings,
        "/" => openapi_get_routes_spec![get_index],
        "/auth" => authentication::routes()
    };
    rocket.mount(
        "/doc",
        make_rapidoc(&rocket_okapi::rapidoc::RapiDocConfig {
            title: Some("Slink API Documentation".into()),
            general: GeneralConfig {
                spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                ..Default::default()
            },
            ..Default::default()
        }),
    )
}
