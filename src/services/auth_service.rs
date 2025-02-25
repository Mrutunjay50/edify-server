use crate::models::{students::Student, teachers::Teacher};
use crate::repository::{student_repository::StudentRepository, teacher_repository::TeacherRepository};
use bcrypt::{hash, verify, DEFAULT_COST};
use mongodb::error::Result;
use async_trait::async_trait;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Serialize, Deserialize};
use std::io::{Error, ErrorKind};

#[derive(Serialize, Deserialize)]
struct Claims {
    user_id: String,
    profession: String,
    exp: usize,
}

#[async_trait]
pub trait AuthService {
    async fn register_student(&self, student: Student) -> Result<String>;
    async fn register_teacher(&self, teacher: Teacher) -> Result<String>;
    async fn login(&self, email: &str, password: &str) -> Result<Option<String>>;
}

pub struct AuthServiceImpl<STUDENT: StudentRepository + Send + Sync, TEACHER: TeacherRepository + Send + Sync> {
    student_repo: STUDENT,
    teacher_repo: TEACHER,
    jwt_secret: String,
}

impl<STUDENT, TEACHER> AuthServiceImpl<STUDENT, TEACHER>
where
    STUDENT: StudentRepository + Send + Sync,
    TEACHER: TeacherRepository + Send + Sync,
{
    pub fn new(student_repo: STUDENT, teacher_repo: TEACHER, jwt_secret: String) -> Self {
        Self {
            student_repo,
            teacher_repo,
            jwt_secret,
        }
    }

    async fn generate_token(&self, user_id: String, profession: String) -> String {
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(8))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            user_id,
            profession,
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .unwrap()
    }
}

#[async_trait]
impl<STUDENT, TEACHER> AuthService for AuthServiceImpl<STUDENT, TEACHER>
where
    STUDENT: StudentRepository,
    TEACHER: TeacherRepository,
{
    async fn register_student(&self, mut student: Student) -> Result<String> {
        student.email = student.email.to_lowercase();
        student.username = student.username.to_uppercase();
        
        if self.student_repo.find_by_email(&student.email).await?.is_some() {
            return Err(mongodb::error::Error::from(std::io::Error::new(std::io::ErrorKind::Other, "Email already exists")));
        }
        
        // Ensure password is present before hashing
        let password = student.password.as_ref()
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "Password is required"))?;
            student.password = Some(hash(password, DEFAULT_COST).map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?);

        self.student_repo.create_student(&student).await?;

        // Ensure `generate_token` returns a valid string
        // Ensure `id` exists before using it
        let student_id = student.id
        .expect("Student ID should be set after creation")
        .to_hex();
    
        let token = self.generate_token(student_id, "STUDENT".to_string()).await;
        
        Ok(token)
    
    }

    async fn register_teacher(&self, mut teacher: Teacher) -> Result<String> {
        teacher.email = teacher.email.to_lowercase();
        teacher.username = teacher.username.to_uppercase();
        
        if let Some(existing) = self.teacher_repo.find_by_email(&teacher.email).await? {
            return Err(mongodb::error::Error::from(std::io::Error::new(std::io::ErrorKind::Other, "Email already exists")));
        }

        // Ensure password is present before hashing
        let password = teacher.password.as_ref()
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "Password is required"))?;
            teacher.password = Some(hash(password, DEFAULT_COST).map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?);

        self.teacher_repo.create_teacher(&teacher).await?;

        // Ensure `generate_token` returns a valid string
        // Ensure `id` exists before using it
        let teacher_id = teacher.id
        .expect("teacher ID should be set after creation")
        .to_hex();
    
        let token = self.generate_token(teacher_id, "TEACHER".to_string()).await;
        
        Ok(token)
    }

    async fn login(&self, email: &str, password: &str) -> Result<Option<String>> {
        // let user_student = self.student_repo.find_by_email(email).await?;
        // let user_teacher = self.teacher_repo.find_by_email(email).await?;

        // if let Some(user) = user_student.or(user_teacher) {
        //     if let Some(stored_password) = &user.password {
        //         if verify(password, stored_password).unwrap() {
        //             let profession = if user_student.is_some() { "STUDENT" } else { "TEACHER" };
        //             return Ok(Some(self.generate_token(user.id.to_hex(), profession.to_string()).await));
        //         }
        //     }
        //     return Ok(None);
        // }

        Ok(None)
    }
}
