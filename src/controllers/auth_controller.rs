use actix_web::{HttpResponse, Responder, post, web};
use mongodb::bson::doc;

use crate::{
    db::db::Database,
    interfaces::register_request::{LoginRequest, RegisterRequest},
    models::{
        students::{SocialAccounts, Student},
        teachers::{SocialAccounts as SocialAccountsForTeacher, Teacher},
    }, utils::api_response::ApiResponse,
};

pub async fn register_user(
    db: web::Data<Database>,
    payload: web::Json<RegisterRequest>,
) -> impl Responder {
    let register_data = payload.into_inner();
    let profession = register_data.profession.to_uppercase();
    let username = register_data.username.to_uppercase();
    let email = register_data.email.clone();
    println!("Attempting to register: {:?}", register_data);

    if db.student_repo
    .get_student(doc! {"$or": [{"email": &email.to_lowercase()}, {"username": &username}]})
    .await
    .map(|students| !students.is_empty()) // Convert to bool
    .unwrap_or(false) // Handle errors safely
    || 
    db.teacher_repo
    .get_teacher(doc! {"$or": [{"email": &email.to_lowercase()}, {"username": &username}]})
    .await
    .map(|teachers| !teachers.is_empty()) // Convert to bool
    .unwrap_or(false) // Handle errors safely
{
    return HttpResponse::Conflict().json(ApiResponse::error(409, "User with the same email or username already exists"));
}

    let result = match profession.as_str() {
        "STUDENT" => {
            let student = Student {
                id: None,
                profile_picture: "".to_string(),
                fullname: register_data.fullname,
                username,
                bio: "".to_string(),
                email: email.to_lowercase(),
                password: Some(register_data.password),
                contact: "".to_string(),
                pronoun: "notspecified".to_string(),
                profession: "STUDENT".to_string(),
                age: None,
                socialacc: SocialAccounts {
                    instagram: "".to_string(),
                    twitter: "".to_string(),
                    linkedin: "".to_string(),
                },
                institute: "".to_string(),
                passing_year: "".to_string(),
                in_what: "".to_string(),
                school_student: "".to_string(),
                college_student: "".to_string(),
                recent_items: vec![],
                completed_items: vec![],
                action_scores: 0,
                test_scores: 0,
                total_exp: 0,
            };
            db.student_repo.create_student(student).await
        }
        "TEACHER" => {
            let teacher = Teacher {
                id: None,
                profile_picture: "".to_string(),
                fullname: register_data.fullname,
                username,
                bio: "".to_string(),
                email: email.to_lowercase(),
                password: Some(register_data.password),
                contact: "".to_string(),
                pronoun: "notspecified".to_string(),
                profession: "TEACHER".to_string(),
                age: None,
                socialacc: SocialAccountsForTeacher {
                    instagram: "".to_string(),
                    twitter: "".to_string(),
                    linkedin: "".to_string(),
                },
                experience: "".to_string(),
                classes: "".to_string(),
                subjects: "".to_string(),
            };
            db.teacher_repo.create_teacher(teacher).await
        }
        _ => {
            return HttpResponse::BadRequest()
            .json(ApiResponse::error(400, "Invalid profession. Please specify either 'STUDENT' or 'TEACHER'."));
        }
    };

    match result {
        Ok(insert_result) => HttpResponse::Created().json(ApiResponse::success(
            201,
            "User registered successfully",
            format!("User ID: {:?}", insert_result.inserted_id),
        )),
        Err(e) => {
            println!("Error registering user: {:?}", e);
            HttpResponse::InternalServerError()
                .json(ApiResponse::error(500, "An error occurred while processing the request. Please try again."))
        }
    }
}

pub async fn login_user(
    db: web::Data<Database>,
    payload: web::Json<LoginRequest>,
) -> impl Responder {
    // let login_data = payload.into_inner();
    // let token_result = db.login(&login_data.email, &login_data.password).await;

    // match token_result {
    //     Ok(Some(token)) => HttpResponse::Ok().json(token),
    //     Ok(None) => HttpResponse::Unauthorized().json("Invalid credentials"),
    //     Err(_) => HttpResponse::InternalServerError().json("Error processing request"),
    // }
    HttpResponse::Unauthorized().json("Invalid credentials")
}
