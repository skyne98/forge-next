use anyhow::Result;
use server::Server;
use std::env;

pub mod data;
pub mod handlers;
pub mod protocol;
pub mod server;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = env_logger::try_init();
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let server = Server::new(addr).await?;
    server.listen().await?;

    Ok(())
}
