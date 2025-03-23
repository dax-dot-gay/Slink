use bson::Uuid;
use chrono::Utc;
use manor::{Collection, Model};
use rocket::{
    Data, Request, Response,
    fairing::{Fairing, Info, Kind},
    http::Cookie,
};
use slink_common::types::AppConfig;

use crate::models::Session;

pub struct SessionFairing;

#[async_trait]
impl Fairing for SessionFairing {
    fn info(&self) -> rocket::fairing::Info {
        Info {
            name: "Session Manager",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        let config = req
            .rocket()
            .figment()
            .extract_inner::<AppConfig>("slink")
            .unwrap();
        if let Some(token) = req.cookies().get("slink.token") {
            if let Ok(id) = Uuid::parse_str(token.value()) {
                if let Ok(Some(existing)) = Collection::<Session>::new().get(id).await {
                    if existing.last_connection + config.authentication.session_max_lifetime
                        >= Utc::now()
                    {
                        req.local_cache(|| existing);
                        return;
                    } else {
                        let _ = existing.delete().await;
                    }
                }
            }
        }

        let new_session = req.local_cache(|| Session::create());
        new_session.save().await.unwrap();
        return;
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let mut session = req.local_cache(|| Session::create()).clone();
        session.last_connection = Utc::now();
        session.save().await.unwrap();
        res.set_header(Cookie::new("slink.token", session.id.to_string()));
    }
}
