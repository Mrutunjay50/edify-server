use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use env_logger;
use std::env;

mod db;

async fn running_status() -> impl Responder {
    println!("Hit: /running-status");  // <-- Manually log route hits
    HttpResponse::Ok().body("API is connected")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let _mongodb_client = db::connect_to_mongodb()
        .await
        .expect("Failed to connect to MongoDB");

    println!("Server running on port {}", port);
    
    HttpServer::new(|| {
        App::new()
        .wrap(Cors::permissive()) // Allow all origins (for testing)
        .wrap(Logger::default()) // Actix Logger Middleware
        .route("/running-status", web::get().to(running_status))
    })
    .bind(("127.0.0.1", port.parse::<u16>().unwrap()))?
    .run()
    .await
    
}



// $env:RUST_LOG="info"; cargo run     // Run the server with logging enabled
// cargo watch -x "run"
// $env:RUST_LOG="info"; cargo watch -x "run"
