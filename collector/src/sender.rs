// Sending data

use std::collections::VecDeque;
use crate::errors::CollectorError;
use std::io::{Read, Write};
use std::net::TcpStream;
use shared_code::{decode_response_v1, CollectorCommandV1, CollectorResponseV1, DATA_COLLECTOR_ADDRESS};


pub fn send_command(command: &CollectorCommandV1) -> Result<(), CollectorError> {
    let bytes = shared_code::encode_v1(&command);
    println!("Encoded {} bytes", bytes.len());
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS)
        .map_err(|_| CollectorError::UnableToConnect)?;
    stream.write_all(&bytes)
        .map_err(|_| CollectorError::UnableToSendData)?;;

    Ok(())
}
pub fn send_queue(
    queue: &mut VecDeque<Vec<u8>>,
    collector_id: u128
) -> Result<(), CollectorError> {
    // Connect
    let mut stream = TcpStream::connect(DATA_COLLECTOR_ADDRESS)
        .map_err(|_| CollectorError::UnableToConnect)?;
    let mut buf = [0u8; 512];

    // Try sending each queued command, but only pop on a real Ack
    while let Some(command) = queue.front() {
       println!("received command: {:?}", command);
        // send (peek at &Vec<u8>)
        
        if stream.write_all(command).is_err() {
            return Err(CollectorError::UnableToSendData);
        }
        println!("sent command, about to read bytes");
        // read response
        let bytes_read = stream.read(&mut buf)
            .map_err(|_| CollectorError::UnableToReceiveData)?;
        if bytes_read == 0 {
            return Err(CollectorError::UnableToReceiveData);
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
    Ok(())
}