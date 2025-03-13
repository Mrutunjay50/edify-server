use actix_web::{web::{self, Query}, HttpResponse, Responder};
use mongodb::bson::doc;
use serde::Deserialize;
use crate::{db::db::Database, utils::api_response::ApiResponse};
use std::collections::HashSet;

#[derive(Deserialize)]
pub struct CourseQuery {
    coursename: String,
}

pub async fn get_college_course(
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

pub async fn get_school_course(
    db: web::Data<Database>,
) -> impl Responder {
    let course_filter = doc! { "coursename" : { "$nin" : ["btech", "bsc"]} };
    let course = match db.course_repo.get_courses(course_filter).await {
        Ok(course) => course,
        _ => return HttpResponse::Unauthorized().json(ApiResponse::error(401, "No school course found")),
    };

    HttpResponse::Ok().json(ApiResponse::success(
        200,
        "all school courses found successfully",
        serde_json::json!(course),
    ))
}

pub async fn get_particular_course(
    db: web::Data<Database>,
    query: Query<CourseQuery>,
) -> impl Responder {
    let course_filter = doc! { "coursename" : &query.coursename };
    let course = match db.course_repo.get_courses(course_filter).await {
        Ok(course) => course,
        _ => return HttpResponse::Unauthorized().json(ApiResponse::error(401, "No such course found")),
    };

    HttpResponse::Ok().json(ApiResponse::success(
        200,
        "Course found successfully",
        serde_json::json!(course),
    ))
}

pub async fn get_unauthorized_all_course(
    db: web::Data<Database>,
) -> impl Responder {
    let course_filter = doc! {};
    let courses = match db.course_repo.get_courses(course_filter).await {
        Ok(courses) => courses,
        _ => return HttpResponse::Unauthorized().json(ApiResponse::error(401, "No school course found")),
    };

    // Extract unique subject names using HashSet
    let unique_subject_names: HashSet<String> = courses
        .into_iter()
        .flat_map(|course| course.subjects.into_iter().map(|subject| subject.subjectname))
        .collect();

    HttpResponse::Ok().json(ApiResponse::success(
        200,
        "All unique school subjects found successfully",
        serde_json::json!(unique_subject_names),
    ))
}

