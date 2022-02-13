extern crate dotenv;

use actix_web::{App, get, HttpServer, HttpResponse, Responder};
use std::env;
use dotenv::dotenv;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

pub async fn serve() -> std::io::Result<()> {
    dotenv().expect("Failed to read .env file");

    let host = env::var("APP_HOST").expect("APP_HOST not found.");
    let port = env::var("APP_PORT").expect("APP_PORT not found.");
    let address = format!("{}:{}", host, port);

    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
    })
        .bind(address.clone())?;

    println!("Listening in {}", address);

    server.run().await
}