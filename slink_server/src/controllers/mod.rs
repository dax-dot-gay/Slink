use rocket::Rocket;
use rocket_okapi::{mount_endpoints_and_merged_docs, rapidoc::{make_rapidoc, GeneralConfig}, settings::{OpenApiSettings, UrlObject}};

pub mod authentication;

pub fn apply(mut rocket: Rocket<rocket::Build>) -> Rocket<rocket::Build> {
    let settings = OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
        rocket, "/".to_owned(), settings,
        "/auth" => authentication::routes()
    };
    rocket.mount("/doc", make_rapidoc(&rocket_okapi::rapidoc::RapiDocConfig { title: Some("Slink API Documentation".into()), general: GeneralConfig {
        spec_urls: vec![UrlObject::new("General", "../openapi.json")],
        ..Default::default()
    }, ..Default::default() }))
}