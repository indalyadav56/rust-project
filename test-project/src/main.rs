mod db;
mod repositories;
mod services;
mod models;
mod routes;
mod handlers;
mod middleware;

use std::sync::Arc;

use actix_web::middleware::from_fn;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use services::user_service::UserServiceImp;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use repositories::user_repository::UserRepositoryImp;
use crate::repositories::user_repository::UserRepository;
use crate::services::user_service::UserService;

use db::db::init_db;


#[derive(OpenApi)]
#[openapi(
    paths(
      
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints")
    )
)]
struct ApiDoc;


struct AppState {
    user_service: Arc<dyn UserService>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = init_db().await.unwrap();

    // repositories
    let user_repository = UserRepositoryImp::new(pool.clone());
    user_repository.create_user().await.unwrap();

    // services
    let user_service = UserServiceImp::new(Arc::new(user_repository));
    user_service.create_user().await;

    let state = web::Data::new(AppState {
        user_service: Arc::new(user_service),
    });

    HttpServer::new(move || {
        App::new()
         .app_data(state.clone())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi())
            )
            .service(web::resource("/").to(index))
            .configure(routes::auth::auth_routes_config)
            .configure(routes::user_routes::user_routes_config)
            .wrap(actix_web::middleware::Logger::default())
            .wrap(from_fn(middleware::auth::my_middleware))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}
