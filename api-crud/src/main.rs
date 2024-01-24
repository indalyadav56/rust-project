mod user;

use crate::user::models::user_model;


use user::services::user_service::UserService;

use actix_web::{HttpServer, App, get, web, HttpResponse};


#[get("/hello/{name}")] 
async fn greet(name: web::Path<String>) -> HttpResponse {
    // repositories::user_repository::UserRepository.create_record();
    let user_service = UserService::new();
    user_service.create_user();

    let user = user_model::User {
        username: "user1".to_string(),
        email: "user1@example.com".to_string(),
        age: 26,
    };

    let body = serde_json::to_string(&user).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind(("127.0.0.1", 8005))?
    .run()
    .await
}