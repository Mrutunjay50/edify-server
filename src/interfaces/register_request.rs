use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub fullname: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub profession: String,
    pub in_what: Option<String>,       // Now optional
    pub college_student: Option<String>, // Now optional
    pub school_student: Option<String>, // Now optional
    pub classes: Option<String>, // Now optional
}


#[derive(Deserialize)]
pub struct LoginRequest {
    pub email_or_username: String,
    pub password: String,
    pub profession: String,
}