use sqlx::postgres::PgPool;


pub struct UserRepository{
    pool: PgPool
}

impl UserRepository{
    pub fn new(pool: PgPool) -> Self {
        Self {pool: pool}
    }

    pub async fn create_record(&self, name: &str, email: &str){
        println!("Creating record starting... name:- {} and email:- {}", name, email);
        
        let result = sqlx::query!(
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *",
            name,
            email
        ).fetch_one(&self.pool).await;

        println!("{:?}", result);
        println!("indal user created successfully!!");
    }
}