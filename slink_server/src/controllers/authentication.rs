use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use slink_common::ApiResult;
use ts_rs::TS;

use crate::models::{RedactedUser, Session};

#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export)]
pub struct LoginModel {
    pub username: String,
    pub password: String,
}

/*#[post("/login", data = "<login>")]
pub async fn login(session: Session, login: Json<LoginModel>) -> ApiResult<Json<RedactedUser>>*/
