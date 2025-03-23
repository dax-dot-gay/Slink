use bson::Bson;
use manor::{Link, Model};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct TSLink {
    pub collection: String,
    pub id: String,
}

impl<T: Model> From<Link<T>> for TSLink {
    fn from(value: Link<T>) -> Self {
        Self {
            collection: value.collection,
            id: Into::<Bson>::into(value.id).as_str().unwrap().to_string(),
        }
    }
}
