use sqlx::postgres::PgPoolOptions;

pub async fn init_db() -> Result<sqlx::postgres::PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432/postgres")
        .await?;
    
    Ok(pool)
}