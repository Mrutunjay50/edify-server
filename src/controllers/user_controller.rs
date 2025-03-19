use actix_web::{web::{self, Query}, HttpResponse, Responder};
use mongodb::bson::{self, doc};
use serde::Deserialize;
use crate::{db::db::Database, interfaces::{register_request::StudentUpdateRequest, schema_utilities::{EducationLevel, InWhat, Profession}}, models::{students::{SocialAccounts, Student}, teachers::{SocialAccounts as SocialAccountsForTeacher, Teacher},}, utils::api_response::ApiResponse};


pub async fn get_user(
    db: web::Data<Database>,
) -> impl Responder {
    let course_filter = doc! { "coursename" : { "$in" : ["btech", "bsc"]} };
    let course = match db.course_repo.get_courses(course_filter).await {
        Ok(course) => course,
        _ => return HttpResponse::Unauthorized().json(ApiResponse::error(401, "No college course found")),
    };

    HttpResponse::Ok().json(ApiResponse::success(
        200,
        "all college courses found successfully",
        serde_json::json!(course),
    ))
}

pub async fn update_student(
    db: web::Data<Database>,
    user_id: String,
    payload: web::Json<StudentUpdateRequest>,
) -> impl Responder {
    let register_data = payload.into_inner();
    let mut update_fields = doc! {};

    if let Some(profile_picture) = register_data.profile_picture {
        update_fields.insert("profile_picture", profile_picture);
    }
    if let Some(fullname) = register_data.fullname {
        update_fields.insert("fullname", fullname);
    }
    if let Some(username) = register_data.username {
        update_fields.insert("username", username.to_uppercase());
    }
    if let Some(bio) = register_data.bio {
        update_fields.insert("bio", bio);
    }
    if let Some(email) = register_data.email {
        update_fields.insert("email", email.to_lowercase());
    }
    if let Some(password) = register_data.password {
        update_fields.insert("password", password);
    }
    if let Some(contact) = register_data.contact {
        update_fields.insert("contact", contact);
    }
    if let Some(pronoun) = register_data.pronoun {
        update_fields.insert("pronoun", pronoun);
    }
    if let Some(profession) = register_data.profession {
        if profession.to_uppercase() != "STUDENT" {
            return HttpResponse::BadRequest().json(ApiResponse::error(400, "Invalid profession"));
        }
        update_fields.insert("profession", "STUDENT");
    }
    if let Some(age) = register_data.age {
        update_fields.insert("age", age);
    }
    if let Some(socialacc) = register_data.socialacc {
        update_fields.insert("socialacc", bson::to_bson(&socialacc).unwrap());
    }
    
    if let Some(institute) = register_data.institute {
        update_fields.insert("institute", institute);
    }
    if let Some(passing_year) = register_data.passing_year {
        update_fields.insert("passing_year", passing_year);
    }
    update_fields.insert("in_what", match register_data.in_what.unwrap().to_uppercase().as_str() {
        "SCHOOL" => "SCHOOL",
        "COLLEGE" => "COLLEGE",
        _ => return HttpResponse::BadRequest().json(ApiResponse::error(400, "Invalid in_what parameter")),
    });
    if let Some(education_level) = register_data.education_level {
        update_fields.insert("socialacc", bson::to_bson(&education_level).unwrap());
    }
    if let Some(recent_items) = register_data.recent_items {
        update_fields.insert("recent_items", recent_items);
    }
    if let Some(completed_items) = register_data.completed_items {
        update_fields.insert("completed_items", completed_items);
    }
    if let Some(action_scores) = register_data.action_scores {
        update_fields.insert("action_scores", action_scores);
    }
    if let Some(test_scores) = register_data.test_scores {
        update_fields.insert("test_scores", test_scores);
    }
    if let Some(total_exp) = register_data.total_exp {
        update_fields.insert("total_exp", total_exp);
    }

    match db.student_repo.update_student(doc! {"_id" : user_id}, update_fields).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::success(200, "Student updated successfully", "")),
        Err(e) => {
            println!("Error updating student: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::error(500, "Failed to update student"))
        }
    }
}