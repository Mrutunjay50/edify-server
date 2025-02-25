use crate::models::students::Student;
use mongodb::{bson::doc, error::Result, Collection, Database};
use async_trait::async_trait;

#[async_trait]
pub trait StudentRepository: Send + Sync {
    async fn create_student(&self, student: &Student) -> Result<mongodb::results::InsertOneResult>;
    async fn find_by_email(&self, email: &str) -> Result<Option<Student>>;
    async fn find_by_username(&self, username: &str) -> Result<Option<Student>>;
}

pub struct MongoStudentRepository {
    collection: Collection<Student>,
}

impl MongoStudentRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("students"),
        }
    }
}

#[async_trait]
impl StudentRepository for MongoStudentRepository {
    async fn create_student(&self, student: &Student) -> Result<mongodb::results::InsertOneResult> {
        self.collection.insert_one(student).await
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<Student>> {
        self.collection.find_one(doc! { "email": email }).await
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<Student>> {
        self.collection.find_one(doc! { "username": username }).await
    }
}
