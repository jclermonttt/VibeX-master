use anyhow::Result;

pub async fn run() -> Result<()> {
    // Server logic here.
    Ok(())
}

#[tokio::main]
pub async fn main() -> Result<()> {
    run().await
}
