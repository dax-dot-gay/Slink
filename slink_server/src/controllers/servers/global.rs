use okapi::openapi3::OpenApi;
use rocket_okapi::openapi_get_routes_spec;

pub fn routes() -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![]
}
