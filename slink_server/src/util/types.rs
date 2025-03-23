use bson::Bson;
use manor::{Link, Model};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Deserialize, Serialize, Clone, Debug)]
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
