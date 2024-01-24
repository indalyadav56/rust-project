use actix_web::{web::Json, HttpResponse};

pub struct UserHandler; 


impl UserHandler {

    pub async fn get_users() -> HttpResponse {
        HttpResponse::Ok().json("List of users") 
    }

    pub async fn create_user() -> HttpResponse {
        HttpResponse::Created().finish()
    }
}