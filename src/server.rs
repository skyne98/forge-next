use futures_util::{SinkExt, StreamExt};
use log::*;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Message, Result},
};

pub struct Server {
    addr: String,
    listener: TcpListener,
}

impl Server {
    pub async fn new(addr: String) -> Result<Self> {
        // Create the event loop and TCP listener we'll accept connections on.
        let try_socket = TcpListener::bind(&addr).await;
        let listener = try_socket.expect("Failed to bind");

        Ok(Server { addr, listener })
    }

    pub async fn listen(&self) -> Result<()> {
        info!("Listening on: {}", self.addr);
        while let Ok((stream, _)) = self.listener.accept().await {
            let peer = stream
                .peer_addr()
                .expect("connected streams should have a peer address");
            info!("Peer address: {}", peer);

            tokio::spawn(Self::accept_connection(peer, stream));
        }

        Ok(())
    }

    async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
        if let Err(e) = Self::handle_connection(peer, stream).await {
            match e {
                Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                err => error!("Error processing connection: {}", err),
            }
        }
    }

    async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<()> {
        let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

        info!("New WebSocket connection: {}", peer);

        while let Some(msg) = ws_stream.next().await {
            let msg = msg?;
            if msg.is_text() || msg.is_binary() {
                // There is a new message
                if let Message::Binary(bytes) = msg {}
            }
        }

        Ok(())
    }
}
