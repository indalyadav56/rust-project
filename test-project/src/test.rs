use std::{collections::HashMap, string};


#[derive(Debug)]
enum Planet {
    INDAL,
}
// println!("enum example:-> {:?}",Planet::INDAL);

// let greeting = String::from("Hello");

// let closure = move || {
//     // Ownership of `greeting` is moved into the closure
//     println!("{}", greeting);
// };

// println!("{}", greeting); // Error: `greeting` has been moved

// closure(); // The closure can still access `greeting`
fn main() {

    use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            // prefixes all resources and routes attached to it...
            web::scope("/app")
                // ...so this handles requests for `GET /app/index.html`
                .route("/index.html", web::get().to(index)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
}

// let greeting = || {
//     println!("INDAL_KUMAR");
// };
// greeting();

// let math = |x: i32, y: i32, z: i32| -> i32 {
//     let sum = x + y;
//     sum * z
// };

// let result = math(20, 1, 4);
// println!("result: {}", result);


fn print_name(name:&str) {
    // let mut newHasmap = HashMap::new(); 
    // newHasmap.insert("k", "v");
    
    // let mut one_name = "print_name";
    // print_name(one_name);
    
    // println!("one_name: {}", one_name);
    
    // one_name = "hello";
    
    // let str_array:[i32; 10]  = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // println!("str_array: {:?}", str_array);
    
    
    // let new_vector_data: Vec<String> = Vec::new();
    
    // let fruits = vec!["apple", "banana", "cherry", 123123, 123123];
    
    // println!("fruits: {:?} ", fruits);  
    
    // println!("2 print one name {one_name}");
    
    // for i in newHasmap {
    //     println!("{:?}", i);
    // }
    // println!("Hello, world!");
    // move |name| println!("Hello, {}!", name);
    println!("{}", name);
}

