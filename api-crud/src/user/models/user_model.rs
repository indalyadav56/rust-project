use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User{
    pub username: String,
    pub email: String,
    pub age: u16,
}