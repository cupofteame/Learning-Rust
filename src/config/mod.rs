use std::env;
use std::net::ToSocketAddrs;
use crate::error::ServerError;
use dotenv::dotenv;

// Configuration struct to hold our settings
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    // Creates a new ServerConfig with default or environment-provided values
    pub fn new() -> Result<Self, ServerError> {
        // Load .env file if it exists
        dotenv().ok();
        
        let port = env::var("PORT")
            .map_err(ServerError::EnvError)?
            .parse()
            .map_err(ServerError::ParseError)?;

        let host = env::var("HOST")
            .map_err(ServerError::EnvError)?;
        
        // Validate the host & port combo
        let addr = format!("{}:{}", host, port);
        addr.to_socket_addrs()
            .map_err(ServerError::IoError)?;

        Ok(Self { host, port })
    }
} 