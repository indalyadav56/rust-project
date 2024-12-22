use crate::{auth::repositories::auth_repository::AuthRepository, user::services::user_service::{self, UserService}, AppState};

pub struct AuthService{
  auth_repository: AuthRepository
}

impl AuthService{
    pub fn new(repo: AuthRepository) -> Self {
      Self {auth_repository: repo}
    }
    
    pub async fn user_login(&self, email:String, password:String){
      self.auth_repository.login(email, password).await;
    }

    pub fn user_register(&self) {
      println!("Login func calling..")
    }
}
