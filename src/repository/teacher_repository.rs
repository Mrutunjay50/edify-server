use futures_util::TryStreamExt;
use mongodb::{bson::{doc, Document}, error::Result, results::InsertOneResult, Collection, Database};
use crate::models::teachers::Teacher;

pub struct TeacherRepository {
    collection: Collection<Teacher>,
}

impl TeacherRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("teachers"),
        }
    }

    pub async fn create_teacher(&self, teacher: Teacher) -> Result<InsertOneResult> {
        self.collection.insert_one(teacher).await
    }

    pub async fn get_teacher(&self, filter: Document) -> Result<Vec<Teacher>> {
        let mut cursor = self.collection.find(filter).await?;
        let mut teachers = Vec::new();

        while let Some(teacher) = cursor.try_next().await? {
            teachers.push(teacher);
        }
        Ok(teachers)
    }
}
