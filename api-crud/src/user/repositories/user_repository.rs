use crate::AppState;

pub struct UserRepository;

impl UserRepository{
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_record(&self){
        println!("create record in db!!!")
    }
}