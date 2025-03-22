use rocket::{fairing::AdHoc, http::Status, serde::json::Json, Request};
use slink_common::{types::{AppConfig, RequestId}, ApiError, ApiResult};

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
    rocket::build()
        .mount("/", routes![index])
        .attach(AdHoc::on_request("Attach Request ID", |req, _| Box::pin(async move {
            req.local_cache(|| RequestId::new());
        })))
        .register("/", catchers![handle_error])
}
