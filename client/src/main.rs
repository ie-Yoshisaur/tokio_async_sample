use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::task;

async fn client(id: i32) {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    println!("Client {} connected", id);

    let request = "increment\n";
    stream.write_all(request.as_bytes()).await.unwrap();
    println!("Client {} sent request", id);
}

#[tokio::main]
async fn main() {
    let num_clients = 5;

    let mut tasks = Vec::new();
    for i in 0..num_clients {
        let task = task::spawn(client(i));
        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }
}
