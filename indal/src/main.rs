mod models;
mod services;
mod repository;
mod database;
mod api;

use repository::user_repository::UserRepository;
use sqlx::postgres::PgPoolOptions;
use services::user_service::UserService;
use database::db::{init_postgres_db, init_database};


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
   let pool = init_database().await?;

   let user_repo = UserRepository::new(&pool);
//    let new_user_srv = UserService::new(user_repo);

   match user_repo.insert_user().await {
    Ok(_) => println!("User inserted successfully"),
    Err(e) => println!("Failed to insert user: {}", e)
   }
   
    // // Initialize database connection
    // let pool = init_database().await?;
    // let new_user  = User{
    //     first_name:"indal".to_string(),
    //     last_name:"yadav".to_string(),
    //     middle_name:"kumar".to_string(),
    //     email: "john1212@example.com".to_string(),
    //     password:"password".to_string(),
    // };
    
    // match new_user.insert_user(&pool).await {
    //     Ok(_) => println!("User inserted successfully\nNew user: {:?}", new_user),
    //     Err(e) => println!("Failed to insert user: {}", e)
    // }

    // init().await;

    Ok(())
}

// async fn init_database() -> Result<sqlx::postgres::PgPool, sqlx::Error> {
//     let pool = PgPoolOptions::new()
//         .max_connections(5)
//         .connect("postgres://postgres:postgres@localhost:5432/postgres")
//         .await?;
    
//     Ok(pool)
// }

