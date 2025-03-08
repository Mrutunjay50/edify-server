use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Video {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chapter {
    pub unitnumber: Option<i32>,
    pub unitname: String,
    pub videos: Vec<Video>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subject {
    pub subjectname: String,
    pub chapter: Vec<Chapter>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Course {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub coursename: String,
    pub subjects: Vec<Subject>,
}
