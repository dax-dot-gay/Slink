use rocket::http::Header;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestId(Uuid);

impl RequestId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Into<Header<'_>> for RequestId {
    fn into(self) -> Header<'static> {
        Header::new("X-SLR-ID", self.0.to_string())
    }
}
