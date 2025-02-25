use crate::models::teachers::Teacher;
use mongodb::{bson::doc, error::Result, Collection, Database};
use async_trait::async_trait;

#[async_trait]
pub trait TeacherRepository: Send + Sync {
    async fn create_teacher(&self, teacher: &Teacher) -> Result<mongodb::results::InsertOneResult>;
    async fn find_by_email(&self, email: &str) -> Result<Option<Teacher>>;
    async fn find_by_username(&self, username: &str) -> Result<Option<Teacher>>;
}

pub struct MongoTeacherRepository {
    collection: Collection<Teacher>,
}

impl MongoTeacherRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("teachers"),
        }
    }
}

#[async_trait]
impl TeacherRepository for MongoTeacherRepository {
    async fn create_teacher(&self, teacher: &Teacher) -> Result<mongodb::results::InsertOneResult> {
        self.collection.insert_one(teacher).await
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<Teacher>> {
        self.collection.find_one(doc! { "email": email }).await
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<Teacher>> {
        self.collection.find_one(doc! { "username": username }).await
    }
}
