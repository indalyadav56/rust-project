use actix_web::{web, HttpResponse};


use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginBody {
    username: String,
    password: String,
}


pub struct AuthHandler; 

impl AuthHandler {

    pub async fn login(login_body: web::Json<LoginBody>) -> HttpResponse {
        println!("username: {} and password: {}", login_body.username, login_body.password);
        HttpResponse::Ok().json("login user") 
    }
    pub async fn register() -> HttpResponse {
        HttpResponse::Ok().json("register user") 
    }
}