use serde::Deserialize;

use crate::models::students::SocialAccounts;

use super::schema_utilities::EducationLevel;

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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudentUpdateRequest {
    pub profile_picture: Option<String>,
    pub fullname: Option<String>,
    pub username: Option<String>,
    pub bio: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub contact: Option<String>,
    pub pronoun: Option<String>,
    pub profession: Option<String>,
    pub age: Option<i32>,
    pub socialacc: Option<SocialAccounts>,
    pub institute: Option<String>,
    pub passing_year: Option<String>,
    pub in_what: Option<String>,
    pub education_level: Option<EducationLevel>,
    pub recent_items: Option<Vec<String>>,
    pub completed_items: Option<Vec<String>>,
    pub action_scores: Option<i32>,
    pub test_scores: Option<i32>,
    pub total_exp: Option<i32>,
}
// pub struct TeacherUpdateRequest {
//     pub fullname: String,
//     pub username: String,
//     pub email: String,
//     pub password: String,
//     pub profession: String,
//     pub in_what: Option<String>,       // Now optional
//     pub college_student: Option<String>, // Now optional
//     pub school_student: Option<String>, // Now optional
//     pub classes: Option<String>, // Now optional
// }


#[derive(Deserialize)]
pub struct LoginRequest {
    pub email_or_username: String,
    pub password: String,
    pub profession: String,
}