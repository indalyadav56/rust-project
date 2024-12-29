use actix_web::{HttpResponse, Responder};


/// Login user
#[utoipa::path(
   post,
   path = "/v1/auth/login",
   request_body = LoginCredentials,
   responses(
       (status = 200, description = "Login successful", body = String),
       (status = 401, description = "Invalid credentials")
   )
)]
pub async fn login()-> impl Responder{
    HttpResponse::Ok().body("Hello from login page")
}

/// Register user
#[utoipa::path(
   post,
   path = "/v1/auth/register",
   request_body = LoginCredentials,
   responses(
       (status = 200, description = "Login successful", body = String),
       (status = 401, description = "Invalid credentials")
   )
)]
pub async fn register()-> impl Responder{
    HttpResponse::Ok().body("Hello from register page")
}