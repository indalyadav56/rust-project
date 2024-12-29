use std::thread;
use std::time::Duration;

fn main() {
    // let handle = thread::spawn(|| {
    //     println!("Hello from a new thread!");
    //     thread::sleep(Duration::from_secs(30));
    //     println!("Thread finished");
    // });

    // println!("Main thread waiting...");
    // handle.join().unwrap();
    // println!("All threads completed");

    // if get_current_user().is_none() {
    //     println!("is returning none");
    // }

    // let user_data = get_current_user().is_none();

    // match user_data {
    //     true => println!("is returning none"),
    //     false => println!("is returning some"),
    // }

    // Without lifetime annotation - This won't compile
    struct Person <'a>{
        name: &'a str  // Error: missing lifetime specifier
    }

    let person = Person { name: "John Doe" };

    println!("Name: {}", person.name);
   
    
}


struct  User{
    name: String,
    email: String
}


fn get_current_user() -> Option<User> {
    None
}