use shared_data::{CollectorCommandV1, DATA_COLLECTOR_ADDRESS};
use std::io::Write;

use crate::CollectorError;

pub fn send_command(command: CollectorCommandV1) -> Result<(), CollectorError> {
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS)
        .map_err(|_| CollectorError::UnableToConnect)?;

    // Loop over receive
    let bytes = shared_data::encode_v1(command);
    println!("Encoded {} bytes", bytes.len());

    stream.write_all(&bytes).map_err(|_| CollectorError::UnableToConnect)?;
    // End Loop

    Ok(())
}