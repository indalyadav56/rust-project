use crate::user::repositories::user_repository::UserRepository;

pub struct UserService {
  user_repository: UserRepository,
}

impl UserService {
  
  pub fn new(repo: UserRepository) -> Self {
      Self {user_repository: repo}
  }

  pub async fn create_user(&self) {
    self.user_repository.create_record("Indal Yadav", "indal@gmail.com");
    println!("User service create func!!")
  }

  pub fn get_users(&self) {
    println!("User service create func!!")
  }

}
