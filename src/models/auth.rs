use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterUserRequest {
    pub google_auth: Option<bool>,
    pub email: String,
    pub _password: String,
}

#[derive(Deserialize)]
pub struct LoginUserRequest {
    pub _google_access_token: Option<String>,
    pub email: String,
    pub password: String,
}
