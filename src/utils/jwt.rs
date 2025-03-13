use std::env;

use chrono::{Duration, Utc};
use jsonwebtoken::{ encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id : String,
    pub email : String,
    pub username: String,
    pub fullname : String,
    pub profession : String,
    pub exp : usize,
}

//generate jwt token
pub async fn generate_jwt_token(user_id: &str, email: &str, username: &str, fullname: &str, profession: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "super_secret_key".to_string());
    let expiration = Utc::now() + Duration::hours(24);

    let claims = Claims {
        user_id: user_id.to_string(),
        email: email.to_string(),
        username: username.to_string(),
        fullname: fullname.to_string(),
        profession: profession.to_string(),
        exp: expiration.timestamp() as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}


//verify or decode jwt token

// pub fn verify_jwt_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
//     let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "super_secret_key".to_string());

//     let result = decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default())?;

//     Ok(result.claims)
// }