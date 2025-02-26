use futures_util::TryStreamExt;
use mongodb::{bson::Document, error::Result, results::InsertOneResult, Collection, Database};
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
        let mut student_data = self.student.find(filter).await?;
        let mut students = Vec::new();

        while let Some(student) = student_data.try_next().await? {
            students.push(student);
        }
        Ok(students)
    }

    pub async fn _get_one_student(&self, filter: Document) -> Result<Vec<Student>> {
        let student_data = self.student.find_one(filter).await?;
        let mut students = Vec::new();

        let student = student_data.is_some();
        if student {
            students.push(student_data.unwrap());
        }
        Ok(students)
    }
}
