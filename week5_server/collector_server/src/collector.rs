use std::net::SocketAddr;
use shared_data::DATA_COLLECTOR_ADDRESS;
use shared_data::decode_v1;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;

pub async fn data_collector() -> anyhow::Result<()> {
    // Listen for TCP connections
    let listener = TcpListener::bind(DATA_COLLECTOR_ADDRESS).await?;

    // Loop forever
    loop {
        // Wait for a connection
        let (socket, address) = listener.accept().await?;
        tokio::spawn(new_connection(socket, address));
    }

    Ok(())
}

async fn new_connection(mut socket: TcpStream, address: SocketAddr) {
    println!("New connection from {address:?}");
    let mut buf = vec![0u8; 1024];
    loop {
        let n = socket.read(&mut buf).await.expect("Failed to read data from socket");
        if n == 0 {
            println!("No data received - connection closed");
            return;
        }

        println!("Received {n} bytes");
        let received_data = decode_v1(&buf[0..n]);
        println!("Received data: {received_data:?}");
    }
}