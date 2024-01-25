use actix_web::web;

use crate::auth::handlers::auth_handler::AuthHandler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/auth")
            .route("/login", web::post().to(AuthHandler::login))
            .route("/login", web::post().to(AuthHandler::register))
    );
}
