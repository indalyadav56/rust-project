use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User{
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}


// impl User {
//     pub async fn insert_user(&self, pool: sqlx::PgPool) -> Result<Self, sqlx::Error> {
//         sqlx::query_as!(Self,"INSERT INTO users (first_name, middle_name, last_name, email, password) VALUES ($1, $2, $3, $4, $5)")
//         .fetch_one(&pool)
//         .await
//     }
// }

impl User {
    pub async fn insert_user(&self, pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO users (first_name, middle_name, last_name, email, password) 
             VALUES ($1, $2, $3, $4, $5)",
            self.first_name,
            self.middle_name,
            self.last_name,
            self.email,
            self.password
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}