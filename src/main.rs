mod config;
mod error;
mod handlers;
mod models;

use actix_web::{App, HttpServer};
use config::ServerConfig;
use handlers::{echo, hello};

// Main for the HTTP server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = ServerConfig::new()
        .map_err(|e| {
            eprintln!("Configuration error: {}", e);
            eprintln!("Try setting the PORT environment variable: $env:PORT='8088'");
            std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
        })?;

    println!("Server starting at http://{}:{}", config.host, config.port);
    
    let host = config.host.clone();
    let port = config.port;
    
    match HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
    })
    .bind((host.clone(), port)) {
        Ok(server) => {
            server.run().await
        },
        Err(e) => {
            eprintln!("Failed to bind to {}:{} - {}", host, port, e);
            eprintln!("Try running with a different port using: $env:PORT='8088' cargo run");
            Err(e)
        }
    }
} 