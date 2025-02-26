use actix_web::{HttpResponse, Responder, post, web};
use mongodb::bson::doc;

use crate::{
    db::db::Database,
    interfaces::register_request::{LoginRequest, RegisterRequest},
    models::{
        students::{SocialAccounts, Student},
        teachers::{SocialAccounts as SocialAccountsForTeacher, Teacher},
    }, utils::{api_response::ApiResponse, jwt::generate_jwt_token},
};

enum User {
    Student(Student),
    Teacher(Teacher),
}

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
    let login_data = payload.into_inner();
    let identifier = login_data.email_or_username.to_lowercase();
    let password = login_data.password;
    let profession = login_data.profession.to_uppercase();

    let (user, profession) = match profession.as_str() {
        "TEACHER" => {
            let teacher_filter = doc! { "$or": [{"email": &identifier.to_lowercase()}, {"username": &identifier.to_uppercase()}] };
            match db.teacher_repo.get_teacher(teacher_filter).await.ok().and_then(|mut t| t.pop()) {
                Some(teacher) => (User::Teacher(teacher), "TEACHER"),
                None => return HttpResponse::Unauthorized().json(ApiResponse::error(401, "No User found with the provided credentials")),
            }
        }
        "STUDENT" => {
            let student_filter = doc! { "$or": [{"email": &identifier.to_lowercase()}, {"username": &identifier.to_uppercase()}] };
            match db.student_repo.get_student(student_filter).await.ok().and_then(|mut s| s.pop()) {
                Some(student) => (User::Student(student), "STUDENT"),
                None => return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(401, "No User found with the provided credentials")),
            }
        }
        _ => {
            return HttpResponse::BadRequest().json(ApiResponse::error(400, "Invalid profession. Please specify either 'STUDENT' or 'TEACHER'."));
        }
    };

    let is_password_valid = match &user {
        User::Student(student) => student.password.as_ref().map(|p| p == &password).unwrap_or(false),
        User::Teacher(teacher) => teacher.password.as_ref().map(|p| p == &password).unwrap_or(false),
    };

    if !is_password_valid {
        return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(401, "Invalid credentials or password mismatch"));
    }

    // Extract user details for the JWT token
    let (user_id, email, username, fullname, profession) = match &user {
        User::Student(student) => (student.id.clone(), &student.email, &student.username, &student.fullname, profession),
        User::Teacher(teacher) => (teacher.id.clone(), &teacher.email, &teacher.username, &teacher.fullname, profession),
    };

    let token = match generate_jwt_token(&user_id.unwrap().to_string(), email, username, fullname, profession).await {
        Ok(token) => token,
        Err(_) => {
            println!("Error generating JWT token");
            return HttpResponse::InternalServerError()
                .json(ApiResponse::error(500, "An error occurred while generating the JWT token. Please try again."));
        }
    };

    // Return successful response
    HttpResponse::Ok().json(ApiResponse::success(
        200,
        "Login successful",
        serde_json::json!({
            "token": token,
            "user": {
                "id": user_id,
                "fullname": fullname,
                "username": username,
                "email": email,
                "profession": profession
            }
        }),
    ))
}
