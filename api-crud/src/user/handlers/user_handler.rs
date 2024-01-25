use actix_web::{web, HttpResponse};

use crate::AppState;


pub struct UserHandler{
}


impl UserHandler {

    pub async fn get_users(app_state: web::Data<AppState>) -> HttpResponse {
        app_state.user_service.get_users();
        HttpResponse::Ok().json("List of users") 
    }
    
    pub async fn create_user(app_state: web::Data<AppState>) -> HttpResponse {
        println!("User handler called!!!");
        app_state.user_service.create_user().await;
        HttpResponse::Created().finish()
    }
}