use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::mpsc,
    time::sleep,
};
use anyhow::Result;

// Basic async function example
async fn fetch_data(id: u32) -> String {
    // Simulate API call
    sleep(Duration::from_millis(100)).await;
    format!("Data for id: {}", id)
}

// TCP Server example
async fn run_server() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on port 8080");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New connection from: {}", addr);

        // Spawn a new task for each connection
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,  // Connection closed
                    Ok(n) => {
                        // Echo the data back
                        if let Err(e) = socket.write_all(&buf[0..n]).await {
                            eprintln!("Failed to write to socket: {}", e);
                            return;
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to read from socket: {}", e);
                        return;
                    }
                }
            }
        });
    }
}

// Channel example
async fn channel_example() {
    let (tx, mut rx) = mpsc::channel(32);  // Buffer size of 32

    // Spawn producer task
    let producer = tokio::spawn(async move {
        for i in 0..5 {
            tx.send(i).await.unwrap();
            sleep(Duration::from_millis(100)).await;
        }
    });

    // Spawn consumer task
    let consumer = tokio::spawn(async move {
        while let Some(value) = rx.recv().await {
            println!("Received: {}", value);
        }
    });

    // Wait for both tasks to complete
    let _ = tokio::join!(producer, consumer);
}

// Select example
async fn select_example() {
    let (tx1, mut rx1) = mpsc::channel(1);
    let (tx2, mut rx2) = mpsc::channel(1);

    // Spawn two producers
    let producer1 = tokio::spawn(async move {
        sleep(Duration::from_secs(1)).await;
        tx1.send("Message from producer 1").await.unwrap();
    });

    let producer2 = tokio::spawn(async move {
        sleep(Duration::from_secs(2)).await;
        tx2.send("Message from producer 2").await.unwrap();
    });

    tokio::select! {
        msg = rx1.recv() => println!("Got message from rx1: {:?}", msg),
        msg = rx2.recv() => println!("Got message from rx2: {:?}", msg),
    }
}

// Mutex example
use tokio::sync::Mutex;
use std::sync::Arc;

async fn mutex_example() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter_clone = counter.clone();
        handles.push(tokio::spawn(async move {
            let mut lock = counter_clone.lock().await;
            *lock += 1;
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    println!("Final count: {}", *counter.lock().await);
}

// Rate limiting example
use std::collections::VecDeque;

struct RateLimiter {
    times: Mutex<VecDeque<tokio::time::Instant>>,
    max_requests: usize,
    interval: Duration,
}

impl RateLimiter {
    fn new(max_requests: usize, interval: Duration) -> Self {
        RateLimiter {
            times: Mutex::new(VecDeque::new()),
            max_requests,
            interval,
        }
    }

    async fn acquire(&self) -> bool {
        let now = tokio::time::Instant::now();
        let mut times = self.times.lock().await;

        // Remove expired timestamps
        while let Some(time) = times.front() {
            if now.duration_since(*time) > self.interval {
                times.pop_front();
            } else {
                break;
            }
        }

        if times.len() < self.max_requests {
            times.push_back(now);
            true
        } else {
            false
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Concurrent tasks example
    let mut handles = vec![];
    
    for i in 0..3 {
        handles.push(tokio::spawn(async move {
            let result = fetch_data(i).await;
            println!("{}", result);
        }));
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await?;
    }

    // Run channel example
    channel_example().await;

    // Run select example
    select_example().await;

    // Run mutex example
    mutex_example().await;

    // Rate limiter example
    let limiter = Arc::new(RateLimiter::new(3, Duration::from_secs(1)));
    let mut handles = vec![];

    for i in 0..5 {
        let limiter = limiter.clone();
        handles.push(tokio::spawn(async move {
            if limiter.acquire().await {
                println!("Request {} allowed", i);
            } else {
                println!("Request {} rate limited", i);
            }
        }));
    }

    for handle in handles {
        handle.await?;
    }

    // Run server in background
    tokio::spawn(async {
        if let Err(e) = run_server().await {
            eprintln!("Server error: {}", e);
        }
    });

    // Keep main task running
    sleep(Duration::from_secs(60)).await;

    Ok(())
}