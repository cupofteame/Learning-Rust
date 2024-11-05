use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
struct Message {
    content: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[post("/echo")]
async fn echo(message: web::Json<Message>) -> impl Responder {
    HttpResponse::Ok().json(message.0)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "8088".to_string());
    let port: u16 = port.parse().unwrap_or(8088);
    
    let host = "127.0.0.1";
    println!("Server starting at http://{}:{}", host, port);
    
    match HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
    })
    .bind((host, port)) {
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