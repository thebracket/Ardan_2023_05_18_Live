#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Applications that receive events need to subscribe
    let subscriber = tracing_subscriber::FmtSubscriber::new();

    // Set the subscriber as the default
    tracing::subscriber::set_global_default(subscriber)?;

    // Log some events
    tracing::info!("Starting up");
    tracing::warn!("Are you sure this is a good idea?");
    tracing::error!("This is an error!");

    Ok(())
}