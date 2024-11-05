use actix_web::{get, post, web, HttpResponse, Responder};
use crate::models::Message;

// Basic handler for root endpoint
#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

// Echo handler that returns the received message
#[post("/echo")]
pub async fn echo(message: web::Json<Message>) -> impl Responder {
    HttpResponse::Ok().json(message.0)
} 