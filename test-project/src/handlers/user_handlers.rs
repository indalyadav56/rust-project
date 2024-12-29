use actix_web::{HttpResponse, web};

use crate::AppState;

pub async fn create_user(app_state: web::Data<AppState>) -> HttpResponse {
    app_state.user_service.create_user().await;
    HttpResponse::Ok().body("User created")
}

pub async fn get_user(app_state: web::Data<AppState>) -> HttpResponse {
    app_state.user_service.create_user().await;
    HttpResponse::Ok().body("User get successfully")
}