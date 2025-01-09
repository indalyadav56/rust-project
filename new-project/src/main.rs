// use reqwest::{Client, Request};

// #[tokio::main]
// async fn main() -> Result<()> {
//     let client = Client::builder()
//         .build()?;

//     let res = client
//         .get("https://api.example.com/data")
//         .header("Authorization", "Bearer YOUR_TOKEN")
//         .send()
//         .await?;

//     println!("Status: {}", res.status());
//     println!("Headers:\n{:#?}", res.headers());

//     let body = res.text().await?;
//     println!("Body:\n{}", body);

//     Ok(())
// }

// fn intercept(mut req: Request) -> Result<Request, Box<dyn std::error::Error>> {
//     req.headers_mut().insert(
//         "User-Agent",
//         "My Custom User Agent".parse().unwrap(),
//     );
//     Ok(req)
// }

use redis::AsyncCommands;
use tokio;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    // Connect to the Redis server
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_async_connection().await?;

    // Set a key in Redis
    con.set("my_key", 42).await?;

    // Get the value of the key
    let value: i32 = con.get("my_key").await?;
    println!("The value of 'my_key' is: {}", value);

    Ok(())
}
