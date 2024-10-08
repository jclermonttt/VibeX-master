use anyhow::{Context, Result};
use tokio::process::Command;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let services = vec![
        ("services/service-auth", "service-auth-server"),
        ("services/service-auth", "service-auth-client"),
        ("services/service-user", "service-user-server"),
        ("services/service-user", "service-user-client"),
    ];

    for (service, binary) in &services {
        let mut cmd = Command::new("cargo");
        let service = service.to_string();
        let binary = binary.to_string();

        cmd.arg("run")
            .arg("--manifest-path")
            .arg(format!("./{}/Cargo.toml", service))
            .arg("--bin")
            .arg(binary);

        tokio::spawn(async move {
            match cmd.spawn() {
                Ok(mut child) => {
                    if let Err(e) = child.wait().await {
                        error!("Failed to wait on child process: {:?}", e);
                    }
                }
                Err(e) => {
                    error!("Failed to start service: {:?}", e);
                }
            }
        });
    }

    tokio::signal::ctrl_c()
        .await
        .context("Failed to listen for event")?;

    info!("Shutting down...");

    Ok(())
}
