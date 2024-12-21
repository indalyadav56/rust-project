use sqlx::postgres::PgPoolOptions;


pub async fn init_postgres_db()-> Result<sqlx::PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432/postgres")
        .await?;
    
    Ok(pool)
}

pub async fn init_database() -> Result<sqlx::postgres::PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432/postgres")
        .await?;
    
    Ok(pool)
}