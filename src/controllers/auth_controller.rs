use actix_web::{post, web, HttpResponse, Responder};

use crate::{interfaces::register_request::{LoginRequest, RegisterRequest}, models::{students::{SocialAccounts, Student}, teachers::{SocialAccounts as SocialAccountsForTeacher, Teacher}}, services::auth_service::AuthService};

pub async fn register_user(
    auth_service: web::Data<Box<dyn AuthService>>,
    payload: web::Json<RegisterRequest>,
) -> impl Responder {
    let register_data = payload.into_inner();
    println!("Registering student: {:?}", register_data);

    let result = if register_data.profession.to_uppercase() == "STUDENT" {
        auth_service
        .register_student(Student {
            id: None,  // ObjectId will be assigned later by MongoDB
            profile_picture: "".to_string(),
            fullname: register_data.fullname,
            username: register_data.username,
            bio: "".to_string(),
            email: register_data.email,
            password: Some(register_data.password),
            contact: "".to_string(),
            pronoun: "notspecified".to_string(),
            profession: "STUDENT".to_string(),
            age: None,  // Optional
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
        }).await
    } else if register_data.profession.to_uppercase() == "TEACHER" {
        auth_service
        .register_teacher(Teacher {
            id: None,
            profile_picture: "".to_string(),
            fullname: register_data.fullname,
            username: register_data.username,
            bio: "".to_string(),
            email: register_data.email,
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
        })
        .await
    } else {
        return HttpResponse::BadRequest().json("Invalid profession");
    };

    match result {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::InternalServerError().json(e.to_string())
        },
    }
    
}

pub async fn login_user(
    auth_service: web::Data<Box<dyn AuthService>>,
    payload: web::Json<LoginRequest>,
) -> impl Responder {
    let login_data = payload.into_inner();
    let token_result = auth_service.login(&login_data.email, &login_data.password).await;

    match token_result {
        Ok(Some(token)) => HttpResponse::Ok().json(token),
        Ok(None) => HttpResponse::Unauthorized().json("Invalid credentials"),
        Err(_) => HttpResponse::InternalServerError().json("Error processing request"),
    }
}
