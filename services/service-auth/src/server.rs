use anyhow::{Ok, Result};

pub async fn run() -> Result<()> {
    // Your server logic here.
    Ok(())
}

#[tokio::main]
pub async fn main() -> Result<()> {
    run().await
}
