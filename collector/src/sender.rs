// Sending data

use std::collections::VecDeque;
use thiserror::Error;
use std::io::{Read, Write};
use std::net::TcpStream;
use shared_code::{decode_response_v1, CollectorCommandV1, CollectorResponseV1, DATA_COLLECTOR_ADDRESS};

#[derive(Debug, Error)]
pub enum CollectionError {
    #[error("Unable to connect to the server")]
    UnableToConnect,
    #[error("Sending data failed")]
    UnableToSendData,
    #[error("Receiving data failed")]
    UnableToReceiveData

}

pub fn send_command(command: &CollectorCommandV1) -> Result<(), CollectionError> {
    let bytes = shared_code::encode_v1(&command);
    println!("Encoded {} bytes", bytes.len());
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS)
        .map_err(|_| CollectionError::UnableToConnect)?;
    stream.write_all(&bytes)
        .map_err(|_| CollectionError::UnableToSendData)?;;

    Ok(())
}
pub fn send_queue(
    queue: &mut VecDeque<Vec<u8>>,
    collector_id: u128
) -> Result<(), CollectionError> {
    // Connect
    let mut stream = TcpStream::connect(DATA_COLLECTOR_ADDRESS)
        .map_err(|_| CollectionError::UnableToConnect)?;
    let mut buf = [0u8; 512];

    // Try sending each queued command, but only pop on a real Ack
    while let Some(command) = queue.front() {
       println!("received command: {:?}", command);
        // send (peek at &Vec<u8>)
        
        if stream.write_all(command).is_err() {
            return Err(CollectionError::UnableToSendData);
        }
        println!("sent command, about to read bytes");
        // read response
        let bytes_read = stream.read(&mut buf)
            .map_err(|_| CollectionError::UnableToReceiveData)?;
        if bytes_read == 0 {
            return Err(CollectionError::UnableToReceiveData);
        }
        println!("bytes read: {}", bytes_read);
        // decode
        let timestamp = u32::from_be_bytes([buf[4], buf[5], buf[6], buf[7]]) as u64;
        let (_ts, ack) = decode_response_v1(timestamp, &buf[..bytes_read]);
        println!("received ack: {:?}", ack);
        if ack == CollectorResponseV1::Ack {
            // only now remove it from the queue
            queue.pop_front();
            println!("Ack received at timestamp {}", _ts);
        } else {
            // non‑Ack: stop trying further messages
            println!("Unexpected ack received at timestamp {}", _ts);
            break;
        }
    }

    // Now send your RequestWork as before…
    // …

    Ok(())
}