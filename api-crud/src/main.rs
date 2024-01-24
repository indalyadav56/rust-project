mod user;
mod auth;
mod config;

use std::env;

use actix_web::{HttpServer, App, web::Data,};
use env_logger;
use dotenv::dotenv;

use crate::user::routes::user_router;
use crate::auth::routes::auth_router;
use crate::config::db::DbConnection;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    pub db: Pool<Postgres>
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Error building a connection pool");

    HttpServer::new(move || {
        App::new()
        .app_data(Data::new(AppState { db: pool.clone() }))
        .configure(user_router::config)
        .configure(auth_router::config)
    })
    .bind(("127.0.0.1", 8005))?
    .run()
    .await
}