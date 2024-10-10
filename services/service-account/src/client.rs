use anyhow::Result;

pub async fn run() -> Result<()> {
    // Your client logic here.
    Ok(())
}

/// Main entry point of the `Service-Account` client.
///
/// This function is the main entry point and is responsible for calling the `run` function for the Service-Account client.
/// It uses the `tokio::main` attribute to run the client asynchronously.
///
/// # Returns
///
/// This function returns a `Result` indicating the success or failure of the client operations.
///
/// # Errors
///
/// This function will return an error if the client encounters any issues during runtime.
#[tokio::main]
pub async fn main() -> Result<()> {
    run().await
}
