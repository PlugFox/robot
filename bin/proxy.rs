use color_eyre::Result;
use robot::network::TcpProxy;
use tracing::Level;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // Initialize tracing
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Parse command line arguments or use defaults
    let listen_addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6900".to_string());

    let target_addr = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "127.0.0.1:6121".to_string());

    println!("🤖 Starting Ragnarok Online TCP Proxy");
    println!("📡 Listen address: {}", listen_addr);
    println!("🎯 Target server: {}", target_addr);
    println!("⚡ Press Ctrl+C to stop the proxy");
    println!();

    let proxy = TcpProxy::new(listen_addr, target_addr);
    proxy.start().await?;

    Ok(())
}
