use anyhow::Result;

pub async fn run() -> Result<()> {
    // Client logic here.
    Ok(())
}


#[tokio::main]
pub async fn main() -> Result<()> {
    run().await
}