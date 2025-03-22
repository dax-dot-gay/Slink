use chrono::{DateTime, Utc};
use manor::{Link, schema};
use slink_common::ApiResult;
use uuid::Uuid;

use crate::util::security::HashedPassword;

#[schema(collection = "sessions")]
pub struct Session {
    #[field(id = Uuid::new_v4)]
    pub id: Uuid,

    pub created: DateTime<Utc>,
    pub last_connection: DateTime<Utc>,

    #[serde(default)]
    pub user: Option<Link<User>>,
}

impl Session {
    pub fn create() -> Self {
        Session {
            id: Uuid::new_v4(),
            created: Utc::now(),
            last_connection: Utc::now(),
            user: None,
            _collection: None,
        }
    }
}

#[schema(collection = "users")]
pub struct User {
    #[field(id = Uuid::new_v4)]
    pub id: Uuid,
    pub username: String,
    pub hashed_password: HashedPassword,
}

impl User {
    pub fn create(username: impl Into<String>, password: impl Into<String>) -> ApiResult<Self> {
        Ok(Self {
            id: Uuid::new_v4(),
            username: username.into(),
            hashed_password: HashedPassword::new(password)?,
            _collection: None,
        })
    }
}
