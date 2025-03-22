use slink_common::types::AppConfig;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    let figment = rocket.figment();
    let app_config: AppConfig = figment.extract_inner("slink").expect("Application config (<profile>.slink)");

    println!("{app_config:?}");
    rocket.mount("/", routes![index])
}
