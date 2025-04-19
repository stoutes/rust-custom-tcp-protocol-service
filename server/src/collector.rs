use std::net::{SocketAddr};
use tokio::{net::{TcpListener, TcpStream}, io::AsyncReadExt, io::AsyncWriteExt};
use shared_code::{decode_v1, encode_response_v1, CollectorResponseV1, DATA_COLLECTOR_ADDRESS};
pub async fn data_collector() -> anyhow::Result<()> {
    // Listen for TCP connections on the data collector address
    let listener = TcpListener::bind(DATA_COLLECTOR_ADDRESS).await?;

    // Loop forever, accepting connections
    loop {
        // Wait for a new connection
        let (socket, address) = listener.accept().await?;
        tokio::spawn(new_connection(socket, address));
    }
}

async fn new_connection(mut socket: TcpStream, address: SocketAddr) {
    println!("New connection from {address:?}");
    let mut buf = vec![0u8; 1024];

    loop {
        let n = match socket.read(&mut buf).await {
            Ok(0) => {
                println!("Connection closed by peer");
                return;
            }
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error reading from socket: {e}");
                return;
            }
        };

        println!("Received {n} bytes");
        let (timestamp, cmd) = decode_v1(&buf[..n]);
        println!("Decoded @ {timestamp:?}: {cmd:?}");

        // **Send back an Ack frame**
        let ack_bytes = encode_response_v1(&CollectorResponseV1::Ack);
        if let Err(e) = socket.write_all(&ack_bytes).await {
            eprintln!("Failed to send Ack: {e}");
            return;
        }
        // loop back to read next command…
    }
}