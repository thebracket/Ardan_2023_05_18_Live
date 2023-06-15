use shared_data::CollectorCommandV1;
use thiserror::Error;
mod sender;
mod data_collector;

#[derive(Debug, Error)]
pub enum CollectorError {
    #[error("Unable to connect to the server")]
    UnableToConnect,
}

fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<CollectorCommandV1>();

    // Start the collector thread
    let collector_thread = std::thread::spawn(move || {
        data_collector::collect_data(tx);
    });

    // Listen for commands to send
    while let Ok(command) = rx.recv() {
        let _ = sender::send_command(command);
    }

    let _ = collector_thread.join();
}
