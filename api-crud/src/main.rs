mod user;
mod auth;
mod config;

use actix_web::{HttpServer, App, web::Data,};
use env_logger;
use dotenv::dotenv;


use user::{repositories::user_repository::UserRepository, services::user_service::UserService};
use auth::services::auth_service::AuthService;
use crate::user::routes::user_router;
use crate::auth::routes::auth_router;
use crate::config::db;

pub struct AppState {
    user_service: UserService,
    auth_service: AuthService,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    
    let db_pool = db::establish_connection().await;

    HttpServer::new(move || {
        App::new()
        .app_data(Data::new(AppState{
            user_service: UserService::new(UserRepository::new(db_pool.clone())),
            auth_service: AuthService::new(),
        }))
        .configure(user_router::config)
        .configure(auth_router::config)
    })
    .bind(("127.0.0.1", 8005))?
    .run()
    .await
}