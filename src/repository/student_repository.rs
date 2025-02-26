use futures_util::TryStreamExt;
use mongodb::{bson::{doc, Document}, error::Result, results::InsertOneResult, Collection, Database};
use crate::models::students::Student;

pub struct StudentRepository {
    student: Collection<Student>,
}

impl StudentRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            student: db.collection("students"),
        }
    }

    pub async fn create_student(&self, student: Student) -> Result<InsertOneResult> {
        self.student.insert_one(student).await
    }

    pub async fn get_student(&self, filter: Document) -> Result<Vec<Student>> {
        let mut cursor = self.student.find(filter).await?;
        let mut students = Vec::new();

        while let Some(student) = cursor.try_next().await? {
            students.push(student);
        }
        Ok(students)
    }
}
