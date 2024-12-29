use actix_web::web;
use crate::handlers::auth_handlers;

pub fn auth_routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/auth")
            .route("/login", web::post().to(auth_handlers::login))
            .route("/register", web::post().to(auth_handlers::register))
    );
}