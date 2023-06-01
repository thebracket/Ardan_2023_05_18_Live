async fn divide(number: u32, divisor: u32) -> anyhow::Result<u32> {
    if divisor == 0 {
        anyhow::bail!("Dividing by zero is a bad idea")
    } else {
        Ok(number / divisor)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    divide(5, 0).await?;
    Ok(())
}