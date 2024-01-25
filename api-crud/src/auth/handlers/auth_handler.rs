use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::AppState;


#[derive(Deserialize)]
pub struct LoginBody {
    username: String,
    password: String,
}


pub struct AuthHandler; 

impl AuthHandler {

    pub async fn login(app_state: web::Data<AppState>) -> HttpResponse {
        app_state.user_service.get_users();
        // println!("username: {} and password: {}", login_body.username, login_body.password);
        HttpResponse::Ok().json("login user") 
    }
    pub async fn register() -> HttpResponse {
        HttpResponse::Ok().json("register user") 
    }
}