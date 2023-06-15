use shared_data::CollectorCommandV1;
use shared_data::CollectorResponseV1;
use shared_data::decode_v1;
use shared_data::DATA_COLLECTOR_ADDRESS;
use shared_data::encode_response_v1;
use sqlx::{Pool, Sqlite};
use tokio::io::AsyncWriteExt;
use std::net::SocketAddr;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

pub async fn data_collector(cnn: Pool<Sqlite>) -> anyhow::Result<()> {
    // Listen for TCP connections
    let listener = TcpListener::bind(DATA_COLLECTOR_ADDRESS).await?;

    // Loop forever
    loop {
        // Wait for a connection
        let cnn = cnn.clone();
        let (socket, address) = listener.accept().await?;
        tokio::spawn(new_connection(socket, address, cnn));
    }

    Ok(())
}

async fn new_connection(mut socket: TcpStream, address: SocketAddr, cnn: Pool<Sqlite>) {
    println!("New connection from {address:?}");
    let mut buf = vec![0u8; 1024];
    loop {
        let n = socket
            .read(&mut buf)
            .await
            .expect("Failed to read data from socket");
        if n == 0 {
            println!("No data received - connection closed");
            return;
        }

        println!("Received {n} bytes");
        let received_data = decode_v1(&buf[0..n]);
        //println!("Received data: {received_data:?}");

        match received_data {
            (_timestamp, CollectorCommandV1::RequestWork(_collector_id)) => {
                // Do something
                let bytes = encode_response_v1(CollectorResponseV1::NoWork);
                socket.write_all(&bytes).await.unwrap();
            }
            (
                timestamp,
                CollectorCommandV1::SubmitData {
                    collector_id,
                    total_memory,
                    used_memory,
                    average_cpu_usage,
                },
            ) => {
                let collector_id = uuid::Uuid::from_u128(collector_id);
                let collector_id = collector_id.to_string();

                let result = sqlx::query("INSERT INTO timeseries (collector_id, received, total_memory, used_memory, average_cpu) VALUES ($1, $2, $3, $4, $5)")
                    .bind(collector_id)
                    .bind(timestamp)
                    .bind(total_memory as i64)
                    .bind(used_memory as i64)
                    .bind(average_cpu_usage)
                    .execute(&cnn)
                    .await;

                if result.is_err() {
                    println!("Error inserting data into the database: {result:?}");
                } else {
                    let ack = CollectorResponseV1::Ack;
                    let bytes = encode_response_v1(ack);
                    socket.write_all(&bytes).await.unwrap();
                }
            }
        }
    }
}
