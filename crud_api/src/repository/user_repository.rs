use crate::models::user_model::User;

pub struct UserRepository {
    db: sqlx::PgPool
}

impl UserRepository{
    pub fn new(pool: &sqlx::PgPool) -> Self {
        UserRepository {
            db: pool.clone()
        }
    }

    pub async fn insert_user(&self) -> Result<(), sqlx::Error> {
        let new_user = User{
            first_name:"indal".to_string(),
            last_name:"yadav".to_string(),
            middle_name:"kumar".to_string(),
            email: "john1212123123@example.com".to_string(),
            password:"password".to_string(),
        };
        sqlx::query!(
            "INSERT INTO users (first_name, middle_name, last_name, email, password) 
             VALUES ($1, $2, $3, $4, $5)",
            new_user.first_name,
            new_user.middle_name,
            new_user.last_name,
            new_user.email,
            new_user.password
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
   
}