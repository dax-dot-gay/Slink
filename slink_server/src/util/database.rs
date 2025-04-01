use std::ops::{Deref, DerefMut};

use manor::{Collection, Model};
use rocket::{
    Request,
    request::{self, FromRequest},
};
use rocket_okapi::request::OpenApiFromRequest;
use slink_common::ApiError;

#[derive(Clone, Debug)]
pub struct Docs<T: Model>(Collection<T>);

#[async_trait]
impl<'r, T: Model> FromRequest<'r> for Docs<T> {
    type Error = ApiError;
    async fn from_request(_req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        request::Outcome::Success(Self(Collection::<T>::new()))
    }
}

impl<'r, T: Model> OpenApiFromRequest<'r> for Docs<T> {
    fn from_request_input(
        _gen: &mut rocket_okapi::r#gen::OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<rocket_okapi::request::RequestHeaderInput> {
        Ok(rocket_okapi::request::RequestHeaderInput::None)
    }
}

impl<T: Model> Deref for Docs<T> {
    type Target = Collection<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Model> DerefMut for Docs<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
