use actix_web::web;

use crate::user::handlers::user_handler::UserHandler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
        .route(web::get().to(UserHandler::get_users))
        .route(web::post().to(UserHandler::create_user))
    );
}
