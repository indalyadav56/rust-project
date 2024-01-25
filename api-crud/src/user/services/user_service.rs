use crate::user::repositories::user_repository::UserRepository;

pub struct UserService {
  user_repository: UserRepository,
}

impl UserService {
  
  pub fn new(repo: UserRepository) -> Self {
      Self {user_repository: repo}
  }

  pub async fn create_user(&self) {
    println!("create super service start...");
    self.user_repository.create_record("Indal Yadav", "indal@gmail.com").await;
    println!("User service create func!!")
  }

  pub fn get_users(&self) {
    println!("User service create func!!")
  }

}
