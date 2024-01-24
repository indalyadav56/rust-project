// // Struct 
// struct User {
//     username: String,
//     email: String,
//     age: u8
// }

// // Enum
// enum Direction {
//     North,
//     South,
//     East,
//     West 
// }


#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}


fn main(){
    // Instantiate and access fields
    let user1 = User {
        email: String::from("john@example.com"), 
        username: String::from("john"),
        active: true,
        sign_in_count: 1,
    };
        // Method 2    
    println!("User struct print:-{user1:?}");

    // let full_name = "indal".to_string();
    // println!("{}", Direction::West.North);

    // let n = 5;

    // if n < 0 {
    //     println!("{} is negative", n);
    // } else if n > 0 {
    //     println!("{} is positive", n);
    // } else {
    //     println!("{} is zero", n);
    // }

    // let mut name = String::new();
    // name.push_str("indal");

    // println!("{}", &name[0..2]);
    
}
