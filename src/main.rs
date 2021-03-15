use std::{env, io::Error};

use futures_util::StreamExt;
use log::info;
use server::Server;
use tokio::net::{TcpListener, TcpStream};

pub mod data;
pub mod handlers;
pub mod protocol;
pub mod server;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = env_logger::try_init();
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let server = Server::new(addr).await?;
    server.listen().await?;

    Ok(())
}
