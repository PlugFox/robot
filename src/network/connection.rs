use color_eyre::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
//use std::sync::Arc;
use tracing::{debug, error, info /* , warn */};

/// Represents a bidirectional connection between client and server
pub struct Connection {
    client: TcpStream,
    server: TcpStream,
}

impl Connection {
    /// Create a new connection
    pub fn new(client: TcpStream, server: TcpStream) -> Self {
        Self { client, server }
    }

    /// Start bidirectional data forwarding
    pub async fn start_forwarding(self) -> Result<()> {
        let (mut client_read, mut client_write) = self.client.into_split();
        let (mut server_read, mut server_write) = self.server.into_split();

        // Forward data from client to server
        let client_to_server = tokio::spawn(async move {
            let mut buffer = [0u8; 4096];
            let mut total_bytes = 0u64;

            loop {
                match client_read.read(&mut buffer).await {
                    Ok(0) => {
                        info!(
                            "📤 Client connection closed (total sent: {} bytes)",
                            total_bytes
                        );
                        break;
                    }
                    Ok(n) => {
                        total_bytes += n as u64;

                        // Log packet data for debugging (first 16 bytes)
                        let preview = &buffer[..n.min(16)];
                        debug!("📤 Client -> Server: {} bytes, data: {:02x?}", n, preview);

                        // Check if this looks like a RO packet (starts with packet ID)
                        if n >= 2 {
                            let packet_id = u16::from_le_bytes([buffer[0], buffer[1]]);
                            debug!("📦 Packet ID: 0x{:04X}", packet_id);
                        }

                        if let Err(e) = server_write.write_all(&buffer[..n]).await {
                            error!("❌ Error writing to server: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        error!("❌ Error reading from client: {}", e);
                        break;
                    }
                }
            }
        });

        // Forward data from server to client
        let server_to_client = tokio::spawn(async move {
            let mut buffer = [0u8; 4096];
            let mut total_bytes = 0u64;

            loop {
                match server_read.read(&mut buffer).await {
                    Ok(0) => {
                        info!(
                            "📥 Server connection closed (total received: {} bytes)",
                            total_bytes
                        );
                        break;
                    }
                    Ok(n) => {
                        total_bytes += n as u64;

                        // Log packet data for debugging (first 16 bytes)
                        let preview = &buffer[..n.min(16)];
                        debug!("📥 Server -> Client: {} bytes, data: {:02x?}", n, preview);

                        // Check if this looks like a RO packet (starts with packet ID)
                        if n >= 2 {
                            let packet_id = u16::from_le_bytes([buffer[0], buffer[1]]);
                            debug!("📦 Packet ID: 0x{:04X}", packet_id);
                        }

                        if let Err(e) = client_write.write_all(&buffer[..n]).await {
                            error!("❌ Error writing to client: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        error!("❌ Error reading from server: {}", e);
                        break;
                    }
                }
            }
        });

        // Wait for either direction to complete
        tokio::select! {
            _ = client_to_server => {
                info!("✅ Client to server forwarding completed");
            }
            _ = server_to_client => {
                info!("✅ Server to client forwarding completed");
            }
        }

        Ok(())
    }
}
