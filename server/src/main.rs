use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

async fn async_increment(counter: Arc<Mutex<i32>>) {
    let mut value = counter.lock().await;
    let current_value = *value;
    println!("Current value: {}", current_value);

    tokio::time::sleep(Duration::from_secs(2)).await;
    *value += 1;

    println!("Incremented value: {}", *value);
}

async fn handle_client(mut stream: tokio::net::TcpStream, counter: Arc<Mutex<i32>>) {
    let mut buffer = [0; 1024];
    loop {
        let n = stream.read(&mut buffer).await.unwrap();
        if n == 0 {
            break;
        }
        let request = String::from_utf8_lossy(&buffer[..n]);
        println!("Received request: {}", request);

        if request.trim() == "increment" {
            tokio::spawn(async_increment(counter.clone()));
            stream.write_all(b"Incrementation started\n").await.unwrap();
        } else {
            stream.write_all(b"Unknown command\n").await.unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server listening on port 8080");

    let counter = Arc::new(Mutex::new(0));

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let counter_clone = counter.clone();
        tokio::spawn(handle_client(stream, counter_clone));
    }
}
