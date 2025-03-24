use bson::doc;
use controllers::apply;
use fern::colors::{self, Color, ColoredLevelConfig};
use futures::{executor::block_on, StreamExt as _};
use log::info;
use manor::{Client, Collection, Model};
use models::User;
use rocket::{fairing::AdHoc, http::Status, Request};
use slink_common::{types::{AppConfig, DatabaseConfig, RequestId}, ApiError};
use util::fairings::SessionFairing;
mod util;
mod controllers;
mod models;

#[macro_use] extern crate rocket;

#[catch(default)]
fn handle_error(status: Status, request: &Request) -> ApiError {
    request.local_cache(|| ApiError::Uncaught(format!("Error occurred, with original status code {status:?}"))).clone()
}

fn setup_logger() -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new().info(Color::Green).debug(Color::White).trace(Color::White).warn(Color::Yellow).error(Color::Red);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                chrono::Local::now().format("%m/%d/%Y %H:%M:%S%.3f"),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("rocket", log::LevelFilter::Warn)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

#[launch]
fn rocket() -> _ {
    setup_logger().unwrap();
    info!("Initializing Slink server");

    let rocket = rocket::build();
    let config: AppConfig = rocket.figment().extract_inner("slink").expect("No application config (<profile>.slink) configured.");
    
    block_on(async {
        match config.database {
            DatabaseConfig::Options { options, database } => Client::connect_with_options(options, database).await,
            DatabaseConfig::Uri { uri, database } => Client::connect_with_uri(uri, database).await
        }.expect("Failed to connect to specified database.").as_global();
    });

    apply(rocket)
        .attach(AdHoc::on_request("Attach Request ID", |req, _| Box::pin(async move {
            req.local_cache(|| RequestId::new());
        })))
        .attach(AdHoc::on_liftoff("Create Admin User", |rocket| Box::pin(async move {
            let conf: AppConfig = rocket.figment().extract_inner("slink").unwrap();
            if let Some((username, password)) = conf.admin_user {
                if let Some(existing) = User::from_username(username.clone()).await {
                    if !existing.superuser {
                        let _ = existing.delete().await;
                    } else {
                        return;
                    }
                }

                let _ = User::create_super(username.clone(), password).unwrap().save().await;
                info!("Created an admin user from config: {username}");
            }

        })))
        .attach(SessionFairing)
        .register("/", catchers![handle_error])
}
