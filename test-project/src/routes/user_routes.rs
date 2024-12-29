use actix_web::web;

use crate::handlers::user_handlers::{create_user, get_user};

pub fn user_routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/users")
            .route("", web::post().to(create_user))
            .route("", web::get().to(get_user))
    );
}

