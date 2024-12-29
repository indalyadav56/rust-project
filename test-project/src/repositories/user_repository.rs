use std::sync::Arc;
use fake::{faker::internet::en::{FreeEmail, Password}, Fake};

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync{
    async fn create_user(&self) -> Result<(), sqlx::Error>;
}

pub struct UserRepositoryImp {
    db: sqlx::PgPool,
}

impl UserRepositoryImp {
    pub fn new(db: sqlx::PgPool) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl UserRepository for UserRepositoryImp {
    async fn create_user(&self) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO users (first_name, last_name, email, password) VALUES ($1, $2, $3, $4)",
            "first_name".to_string(),
            "last_name".to_string(),
            FreeEmail().fake::<String>(),
            "password".to_string(),
        )
        .execute(&self.db).await?;
        Ok(())
    }
}