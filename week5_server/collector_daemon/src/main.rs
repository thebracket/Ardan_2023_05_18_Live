use shared_data::CollectorCommandV1;

mod data_collector;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<CollectorCommandV1>();

    // Start the collector thread
    let collector_thread = std::thread::spawn(move || {
        data_collector::collect_data(tx);
    });

    let _ = collector_thread.join();
}
