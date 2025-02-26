use mongodb::bson::{oid::ObjectId, doc};
use serde::{Deserialize, Serialize};

use crate::interfaces::schema_utilities::{EducationLevel, Profession};


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Teacher {
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
    pub age: Option<u32>,
    pub socialacc: SocialAccounts,
    pub experience: String,
    pub profession: Profession,  // Always "TEACHER"
    pub classes: Vec<EducationLevel>,
    pub subjects: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SocialAccounts {
    pub instagram: String,
    pub twitter: String,
    pub linkedin: String,
}

impl Teacher {
    /// Validate teacher-specific fields
    pub fn _validate(&self) -> Result<(), String> {
        if self.profession != Profession::Teacher {
            return Err("Invalid profession for Teacher".to_string());
        }
        Ok(())
    }
}
