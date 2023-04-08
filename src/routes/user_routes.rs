use crate::controllers::*;
use actix_web::web;

use controllers::user_controller::*;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::post().to(create_user))
    );
}