use std::env;
use sqlx::postgres::{PgPool, PgPoolOptions};


pub async fn establish_connection() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .unwrap()
}
