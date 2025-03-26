use log::debug;
use manor::{Link, Model};
use okapi::openapi3::OpenApi;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use slink_common::{ApiError, ApiResult, Error};

use crate::models::{RedactedUser, Session, User};

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct LoginModel {
    pub username: String,
    pub password: String,
}

#[openapi(tag = "Authentication")]
#[post("/login", data = "<login>")]
pub async fn login(mut session: Session, login: Json<LoginModel>) -> ApiResult<Json<RedactedUser>> {
    if let Some(user) = User::from_username(login.username.clone()).await {
        if user.hashed_password.verify(login.password.clone()) {
            session.user = Some(Link::from(user.clone()));
            session.save().await.or_else(|e| Err::<_, ApiError>(Error::Unexpected(e.to_string()).into()))?;
            debug!("User {} ({}) logged in successfully.", user.username, user.id);

            return Ok(Json(user.redact()));
        }
    }

    Err(ApiError::bad_login())
}

#[openapi(tag = "Authentication")]
#[delete("/login")]
pub async fn logout(mut session: Session, _user: User) -> ApiResult<()> {
    session.user = None;
    let _ = session.save().await;
    Ok(())
}

pub fn routes() -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![login, logout]
}
