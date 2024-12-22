
use crate::repository::user_repository::UserRepository;

pub struct UserService {
    pub user_repository: UserRepository,
}


impl UserService{
    pub fn new(user_repo :UserRepository) -> Self {
        UserService {
            user_repository: user_repo,
        }
    }
}