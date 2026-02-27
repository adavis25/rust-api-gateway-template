use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Debug, Clone)]
pub struct Settings {
    pub server_addr: SocketAddr,
    pub log_level: String,
}

impl Settings {
    pub fn from_env() -> Self {
        // Loads .env if present (no-op otherwise)
        let _ = dotenvy::dotenv();

        let host: IpAddr = std::env::var("HOST")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST));

        let port: u16 = std::env::var("PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(3000);

        let log_level = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

        Self {
            server_addr: SocketAddr::new(host, port),
            log_level,
        }
    }
}