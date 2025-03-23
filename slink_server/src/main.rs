use futures::executor::block_on;
use manor::Client;
use rocket::{fairing::AdHoc, http::Status, serde::json::Json, Request};
use slink_common::{types::{AppConfig, DatabaseConfig, RequestId}, ApiError, ApiResult};
use ts_rs::TS;
use util::fairings::SessionFairing;
mod util;
mod controllers;
mod models;

#[macro_use] extern crate rocket;

#[get("/")]
fn index(config: AppConfig) -> ApiResult<Json<Vec<String>>> {
    println!("{config:?}");
    Ok(Json(vec![String::from("TEST")]))
}

#[catch(default)]
fn handle_error(status: Status, request: &Request) -> ApiError {
    request.local_cache(|| ApiError::Uncaught(format!("Error occurred, with original status code {status:?}"))).clone()
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    let config: AppConfig = rocket.figment().extract_inner("slink").expect("No application config (<profile>.slink) configured.");
    
    block_on(async {
        match config.database {
            DatabaseConfig::Options { options, database } => Client::connect_with_options(options, database).await,
            DatabaseConfig::Uri { uri, database } => Client::connect_with_uri(uri, database).await
        }.expect("Failed to connect to specified database.").as_global();
    });

    println!("{:?}", Client::global());

    rocket.mount("/", routes![index])
        .attach(AdHoc::on_request("Attach Request ID", |req, _| Box::pin(async move {
            req.local_cache(|| RequestId::new());
        })))
        .attach(SessionFairing)
        .register("/", catchers![handle_error])
}
