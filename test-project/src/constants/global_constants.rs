pub mod constants {

    // HTTP Status Codes
    pub const STATUS_OK: u16 = 200;
    pub const STATUS_BAD_REQUEST: u16 = 400;
    pub const STATUS_NOT_FOUND: u16 = 404;

    // API Responses 
    pub mod response_messages {
         pub const ERROR: &str = "An error occurred";
         pub const SUCCESS: &str = "Request completed successfully";
    }

    // Other Constants
    pub const MAX_LIMIT: u32 = 100;
    pub const DEFAULT_TIMEOUT: u64 = 3000;

}


// main.rs

use crate::constants;

fn main() {

  // Access constants
  println!("Status OK = {}", constants::STATUS_OK);
  
  let limit = 10;
  if limit > constants::MAX_LIMIT {
     // handle error
  }

}