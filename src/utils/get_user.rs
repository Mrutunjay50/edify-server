// use mongodb::bson::doc;

// use crate::{db::db::Database, models::{students::Student, teachers::Teacher}};

// enum User {
//     Student(Student),
//     Teacher(Teacher),
// }

// #[warn(private_interfaces)]
// pub async fn find_user_by_id_and_profession(object_id: &str, profession: &str) -> Result<(User, String), String> {
//     let db = Database::get();

//     match profession.to_uppercase().as_str() {
//         "TEACHER" => {
//             let filter = doc! { "_id": object_id };
//             let result = db.teacher_repo.get_teacher(filter).await.map_err(|_| "Database error")?;
//             if let Some(teacher) = result.into_iter().next() {
//                 return Ok((User::Teacher(teacher), "TEACHER".to_string()));
//             }
//         }
//         "STUDENT" => {
//             let filter = doc! { "_id": object_id };
//             let result = db.student_repo.get_student(filter).await.map_err(|_| "Database error")?;
//             if let Some(student) = result.into_iter().next() {
//                 return Ok((User::Student(student), "STUDENT".to_string()));
//             }
//         }
//         _ => return Err("Invalid profession".to_string()),
//     }
//     Err("User not found".to_string())
// }
