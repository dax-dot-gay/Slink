use crate::util::types::TSLink;
use bson::Uuid;
use manor::{Link, schema};
use schemars::JsonSchema;

use super::User;

#[schema(collection = "servers")]
#[derive(JsonSchema)]
pub struct MinecraftServer {
    #[field(id = Uuid::new)]
    #[schemars(with = "uuid::Uuid")]
    pub id: Uuid,
    pub name: String,

    #[schemars(with = "TSLink")]
    pub owner: Link<User>
}
