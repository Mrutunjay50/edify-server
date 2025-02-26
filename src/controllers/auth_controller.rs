use actix_web::{HttpResponse, Responder, web};
use mongodb::bson::doc;

use crate::{
    db::db::Database,
    interfaces::{register_request::{LoginRequest, RegisterRequest}, schema_utilities::{EducationLevel, InWhat, Profession}},
    models::{
        students::{SocialAccounts, Student},
        teachers::{SocialAccounts as SocialAccountsForTeacher, Teacher},
    }, 
    utils::{api_response::ApiResponse, jwt::generate_jwt_token},
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
    let email = register_data.email.to_lowercase().clone();
    println!("Attempting to register: {:?}", register_data);

    if username.is_empty() {
        return HttpResponse::BadRequest()
            .json(ApiResponse::error(400, "Username is required"));
    }

    if email.is_empty() {
        return HttpResponse::BadRequest()
            .json(ApiResponse::error(400, "Email is required"));
    }

    // let student_data = db.student_repo.get_student(doc! {"$or": [{"email": &email}, {"username": &username}]}).await;

    // println!(" Student data: {:?}", student_data);

    if db.student_repo
    .get_student(doc! {"$or": [{"email": &email}, {"username": &username}]})
    .await
    .map(|students| !students.is_empty()) // Convert to bool
    .unwrap_or(false) // Handle errors safely
    || 
    db.teacher_repo
    .get_teacher(doc! {"$or": [{"email": &email}, {"username": &username}]})
    .await
    .map(|teachers| !teachers.is_empty()) // Convert to bool
    .unwrap_or(false) // Handle errors safely
{
    return HttpResponse::Conflict().json(ApiResponse::error(409, "User with the same email or username already exists"));
}

    let result = match profession.as_str() {
        "STUDENT" => {
            let in_what = match register_data.in_what {
                Some(ref in_what) => in_what.to_uppercase(),
                None => {
                    return HttpResponse::BadRequest()
                        .json(ApiResponse::error(400, "The 'in_what' field is required"));
                }
            };
        
            let school_student = register_data.school_student.unwrap_or_default();
            let college_student = register_data.college_student.unwrap_or_default();
        
            if school_student.is_empty() && college_student.is_empty() {
                return HttpResponse::BadRequest()
                    .json(ApiResponse::error(400, "Either 'school_student' or 'college_student' must be provided"));
            }

            let education_level = if let Some(level) = match school_student.as_str() {
                "6" => Some(EducationLevel::Grade6),
                "7" => Some(EducationLevel::Grade7),
                "8" => Some(EducationLevel::Grade8),
                "9" => Some(EducationLevel::Grade9),
                "10" => Some(EducationLevel::Grade10),
                "11" => Some(EducationLevel::Grade11),
                "12" => Some(EducationLevel::Grade12),
                _ => None,
            } {
                Some(level)
            } else if let Some(level) = match college_student.as_str() {
                "BTech" => Some(EducationLevel::BTech),
                "BSc" => Some(EducationLevel::BSc),
                "BA" => Some(EducationLevel::BA),
                "BCom" => Some(EducationLevel::BCom),
                "BBA" => Some(EducationLevel::BBA),
                "BCA" => Some(EducationLevel::BCA),
                _ => None,
            } {
                Some(level)
            } else {
                return HttpResponse::BadRequest()
                    .json(ApiResponse::error(400, "Invalid education level"));
            };
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
                profession: Profession::Student,
                age: None,
                socialacc: SocialAccounts {
                    instagram: "".to_string(),
                    twitter: "".to_string(),
                    linkedin: "".to_string(),
                },
                institute: "".to_string(),
                passing_year: "".to_string(),
                in_what: match in_what.as_str() {
                    "SCHOOL" => InWhat::School,
                    "COLLEGE" => InWhat::College,
                    _ => return HttpResponse::BadRequest().json(ApiResponse::error(400, "Invalid in_what parameter")),
                },
                education_level,
                recent_items: vec![],
                completed_items: vec![],
                action_scores: 0,
                test_scores: 0,
                total_exp: 0,
            };
            db.student_repo.create_student(student).await
        }
        "TEACHER" => {
            let classes = register_data.classes.unwrap_or_default();
            let mut education_levels = vec![];
        
            for class in classes.split(',').map(|s| s.trim()) {
                match class {
                    "6" => education_levels.push(EducationLevel::Grade6),
                    "7" => education_levels.push(EducationLevel::Grade7),
                    "8" => education_levels.push(EducationLevel::Grade8),
                    "9" => education_levels.push(EducationLevel::Grade9),
                    "10" => education_levels.push(EducationLevel::Grade10),
                    "11" => education_levels.push(EducationLevel::Grade11),
                    "12" => education_levels.push(EducationLevel::Grade12),
                    "BTech" => education_levels.push(EducationLevel::BTech),
                    "BSc" => education_levels.push(EducationLevel::BSc),
                    "BA" => education_levels.push(EducationLevel::BA),
                    "BCom" => education_levels.push(EducationLevel::BCom),
                    "BBA" => education_levels.push(EducationLevel::BBA),
                    "BCA" => education_levels.push(EducationLevel::BCA),
                    _ => {} // Ignore invalid entries
                }
            }

            if education_levels.is_empty() {
                return HttpResponse::BadRequest()
                    .json(ApiResponse::error(400, "Invalid education level(s) in classes"));
            }
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
                profession: Profession::Teacher,
                age: None,
                socialacc: SocialAccountsForTeacher {
                    instagram: "".to_string(),
                    twitter: "".to_string(),
                    linkedin: "".to_string(),
                },
                experience: "".to_string(),
                classes: education_levels,
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
    let identifier = login_data.email_or_username;
    let password = login_data.password;
    let profession = login_data.profession.to_uppercase();

    let (user, profession) = match profession.as_str() {
        "TEACHER" => {
            let teacher_filter = doc! { "$or": [{"email": &identifier.to_lowercase()}, {"username": &identifier.to_uppercase()}] };
            match db.teacher_repo.get_teacher(teacher_filter).await.ok().and_then(|mut t| t.pop()) {
                Some(teacher) => (User::Teacher(teacher), "TEACHER"),
                None => return HttpResponse::Unauthorized().json(ApiResponse::error(401, "No Teacher found with the provided credentials")),
            }
        }
        "STUDENT" => {
            let student_filter = doc! { "$or": [{"email": &identifier.to_lowercase()}, {"username": &identifier.to_uppercase()}] };
            match db.student_repo.get_student(student_filter).await.ok().and_then(|mut s| s.pop()) {
                Some(student) => (User::Student(student), "STUDENT"),
                None => return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(401, "No Student found with the provided credentials")),
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
