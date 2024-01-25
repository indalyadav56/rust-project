use sqlx::postgres::PgPool;


pub struct AuthRepository{
    pool: PgPool
}

impl AuthRepository{
    pub fn new(pool: PgPool) -> Self {
        Self {pool}
    }

    pub async fn login(&self, email: String, password: String){
        // let result = sqlx::query!("select * from users where email = '{}'").fetch_one(&self.pool).await;
        
        let user = sqlx::query!(
            "SELECT * FROM users WHERE email = $1", 
            email
        )
        .fetch_one(&self.pool)
        .await;

        println!("login result:- {:?}", user);
    }
}