//use std::sync::Arc;
//use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
//use tokio::sync::mpsc;
use color_eyre::Result;
use tracing::{error, info /*, warn */};

use super::connection::Connection;

/// TCP Proxy server that forwards traffic between client and Ragnarok Online server
pub struct TcpProxy {
    listen_addr: String,
    target_addr: String,
}

impl TcpProxy {
    /// Create a new TCP proxy
    pub fn new(listen_addr: impl Into<String>, target_addr: impl Into<String>) -> Self {
        Self {
            listen_addr: listen_addr.into(),
            target_addr: target_addr.into(),
        }
    }

    /// Start the proxy server
    pub async fn start(&self) -> Result<()> {
        let listener = TcpListener::bind(&self.listen_addr).await?;
        info!("🚀 TCP Proxy listening on {}", self.listen_addr);
        info!("🔄 Forwarding to {}", self.target_addr);

        loop {
            match listener.accept().await {
                Ok((client_socket, client_addr)) => {
                    info!("✅ Client connected from: {}", client_addr);

                    let target_addr = self.target_addr.clone();

                    // Spawn a new task for each client connection
                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_client(client_socket, target_addr).await {
                            error!("❌ Error handling client {}: {}", client_addr, e);
                        }
                        info!("👋 Client {} disconnected", client_addr);
                    });
                }
                Err(e) => {
                    error!("Failed to accept connection: {}", e);
                }
            }
        }
    }

    /// Handle a single client connection
    async fn handle_client(client_socket: TcpStream, target_addr: String) -> Result<()> {
        // Connect to the target server
        match TcpStream::connect(&target_addr).await {
            Ok(server_socket) => {
                info!("🔌 Connected to server: {}", target_addr);

                // Create connections for bidirectional data forwarding
                let connection = Connection::new(client_socket, server_socket);
                connection.start_forwarding().await?;
            }
            Err(e) => {
                error!("❌ Failed to connect to server {}: {}", target_addr, e);
                return Err(e.into());
            }
        }

        Ok(())
    }
}
