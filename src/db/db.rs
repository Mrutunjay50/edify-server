// use std::env;
// use futures_util::TryStreamExt;
// use mongodb::{bson::doc, options::ClientOptions, results::InsertOneResult, Client, Collection};
// use crate::models::{students::{self, Student}, teachers::Teacher};

// pub struct Database {
//     student: Collection<Student>,
//     teacher: Collection<Teacher>,
// }

// impl Database {
//     pub async fn init() -> Self {
//         let database_uri = env::var("MONGODB_URI").expect("MONGODB_URI environment variable not set");
//         let client_options = ClientOptions::parse(&database_uri).await.unwrap();
//         let client = Client::with_options(client_options).unwrap();
//         let db = client.database("edify");

//         let student: Collection<Student> = db.collection("students");
//         let teacher: Collection<Teacher> = db.collection("teachers");
//         println!("Connected to MongoDB");
//         Database {
//             student,
//             teacher,
//         }
//     }

//     pub async fn create_student(&self, students_data:Student) -> Result<InsertOneResult, mongodb::error::Error> {
//         let result = self.student.insert_one(students_data).await.ok().expect(" Failed to insert student");
//         Ok(result)
//     }
//     pub async fn get_student(&self, student_id:&str) -> Result<Vec<Student>, mongodb::error::Error> {
//         let filter = doc! {"_id" : student_id};
//         let mut result = self.student.find(filter).await.ok().expect(" Failed to insert student");
//         let mut students = Vec::new();
    
//         while let Some(student) = result.try_next().await? {
//             students.push(student);
//         }
//         Ok(students)
//     }
//     pub async fn create_teacher(&self, teachers_data:Teacher) -> Result<InsertOneResult, mongodb::error::Error> {
//         let result = self.teacher.insert_one(teachers_data).await.ok().expect(" Failed to insert student");
//         Ok(result)
//     }
//     pub async fn get_teacher(&self, teacher_id:&str) -> Result<Vec<Teacher>, mongodb::error::Error> {
//         let filter = doc! {"_id" : teacher_id};
//         let mut result = self.teacher.find(filter).await.ok().expect(" Failed to insert student");
//         let mut teachers = Vec::new();
    
//         while let Some(teacher) = result.try_next().await? {
//             teachers.push(teacher);
//         }
//         Ok(teachers)
//     }
// }

use std::env;
use mongodb::{options::ClientOptions, Client};
use crate::repository::{student_repository::StudentRepository, teacher_repository::TeacherRepository};

pub struct Database {
    pub student_repo: StudentRepository,
    pub teacher_repo: TeacherRepository,
}

impl Database {
    pub async fn init() -> Self {
        let database_uri = env::var("MONGODB_URI").expect("MONGODB_URI environment variable not set");
        let client_options = ClientOptions::parse(&database_uri).await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        let db = client.database("edify");

        println!("Connected to MongoDB");

        Self {
            student_repo: StudentRepository::new(&db),
            teacher_repo: TeacherRepository::new(&db),
        }
    }
}
