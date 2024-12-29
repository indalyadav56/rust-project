use std::sync::Arc;

use crate::repositories::user_repository::UserRepository;

#[async_trait::async_trait]
pub trait UserService: Send + Sync {
    async fn create_user(&self);
}

pub struct UserServiceImp {
    repository: Arc<dyn UserRepository>,
}

impl UserServiceImp {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        UserServiceImp { repository }
    }
}

#[async_trait::async_trait]
impl UserService for UserServiceImp {
    async fn create_user(&self){
        self.repository.create_user().await;
    }
}
