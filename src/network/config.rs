use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub listen_addr: String,
    pub target_addr: String,
    pub log_packets: bool,
    pub buffer_size: usize,
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            listen_addr: "127.0.0.1:6900".to_string(),
            target_addr: "127.0.0.1:6121".to_string(),
            log_packets: true,
            buffer_size: 4096,
        }
    }
}
