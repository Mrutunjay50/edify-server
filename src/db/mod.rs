// use mongodb::{Client, options::ClientOptions};
// use std::env;
pub mod db;

// pub async fn connect_to_mongodb() -> mongodb::error::Result<Client> {
//     let database_url = env::var("MONGODB_URI").expect("MONGODB_URI environment variable not set");
//     let client_options = ClientOptions::parse(&database_url).await?;
//     let client = Client::with_options(client_options)?;
//     println!("Connected to MongoDB");
//     Ok(client)
// }