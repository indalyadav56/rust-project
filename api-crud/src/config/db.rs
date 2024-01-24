use std::env;
use sqlx::postgres::{PgPool, PgPoolOptions};


async fn establish_connection() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5) // Adjust as needed
        .connect(&database_url)
        .await?;
    Ok(pool)
}

pub struct DbConnection {
    pool: PgPool,
}

impl DbConnection {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let pool: sqlx::Pool<sqlx::Postgres> = establish_connection().await?;
        Ok(Self { pool })
    }
}


async fn some_function(){

}