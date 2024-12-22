use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::AppState;


#[derive(Serialize, Deserialize)]
pub struct LoginReqBody {
    email: String,
    password: String,
}


pub struct AuthHandler;

impl AuthHandler {
    pub async fn login(req_body: web::Json<LoginReqBody>, app_state: web::Data<AppState>) -> HttpResponse {
        app_state.auth_service.user_login(req_body.email.to_string(), req_body.password.to_string()).await;
        HttpResponse::Ok().json(req_body)
    }

    pub async fn register(app_state: web::Data<AppState>) -> HttpResponse {
        app_state.auth_service.user_register();
        HttpResponse::Ok().json("register user") 
    }
}