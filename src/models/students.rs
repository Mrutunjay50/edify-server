use mongodb::bson::{oid::ObjectId, doc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Student {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub profile_picture: String,
    pub fullname: String,
    pub username: String,
    pub bio: String,
    pub email: String,
    pub password: Option<String>,
    pub contact: String,
    pub pronoun: String,
    pub profession: String,  // Always "STUDENT"
    pub age: Option<u32>,
    pub socialacc: SocialAccounts,
    pub institute: String,
    pub passing_year: String,
    pub in_what: String, // "school" or "college"
    pub school_student: String, // "6" to "12" or ""
    pub college_student: String, // "Btech", "Bsc", or ""
    pub recent_items: Vec<ObjectId>, // Up to 5 items
    pub completed_items: Vec<ObjectId>,
    pub action_scores: i32,
    pub test_scores: i32,
    pub total_exp: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SocialAccounts {
    pub instagram: String,
    pub twitter: String,
    pub linkedin: String,
}

impl Student {
    /// Validate student-specific fields
    pub fn validate(&self) -> Result<(), String> {
        if self.profession != "STUDENT" {
            return Err("Invalid profession for Student".to_string());
        }
        if self.in_what == "school" && self.school_student.is_empty() {
            return Err("School student grade is required".to_string());
        }
        if self.in_what == "college" && self.college_student.is_empty() {
            return Err("College student field is required".to_string());
        }
        if self.recent_items.len() > 5 {
            return Err("Recent items cannot exceed 5".to_string());
        }
        Ok(())
    }
}
