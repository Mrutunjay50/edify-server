use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use env_logger;
use std::env;

mod db;
mod routes;
mod controllers;
mod models;
mod services;
mod repository;
mod interfaces;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());

    let mongodb_client = db::connect_to_mongodb()
        .await
        .expect("Failed to connect to MongoDB");

    println!("Server running on port {}", port);
    
    HttpServer::new(move || {
        App::new()
        .wrap(Cors::permissive()) // Allow all origins (for testing)
        .wrap(Logger::default()) // Actix Logger Middleware
        .app_data(mongodb_client.clone())
        .service(web::scope("/api").configure(routes::configure_routes))
        // .route("/running-status", web::get().to(running_status))
    })
    .bind(("127.0.0.1", port.parse::<u16>().unwrap()))?
    .run()
    .await
    
}



// $env:RUST_LOG="info"; cargo run     // Run the server with logging enabled
// cargo watch -x "run"
// $env:RUST_LOG="info"; cargo watch -x "run"
