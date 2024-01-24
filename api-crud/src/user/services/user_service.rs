use crate::user::repositories::user_repository::UserRepository;

pub struct UserService {
  user_repository: UserRepository,
}

impl UserService {
  
  pub fn new() -> Self {
      let repo = UserRepository::new();
      Self {user_repository: repo}
  }
  pub fn create_user(&self) {
    println!("User service create func!!")
  }

}
