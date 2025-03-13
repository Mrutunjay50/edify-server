use std::env;

use actix_web::{middleware, web};

use crate::{controllers::course_controller::{get_college_course, get_particular_course, get_school_course, get_unauthorized_all_course}, middleware::auth_middleware::JwtAuthMiddleware};


pub fn course_routes(cfg: &mut web::ServiceConfig) {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "super_secret_key".to_string());

    // 🔓 Unprotected route (outside of the authenticated scope)
    cfg.service(
        web::resource("/external/v1/course/get-all-course-overview")
            .wrap(middleware::Logger::default()) // Keep logging middleware
            .route(web::get().to(get_unauthorized_all_course))
    );

    cfg.service(
        web::scope("/external/v1/course")
            .wrap(middleware::Logger::default()) // Optional: Logging middleware
            .wrap(JwtAuthMiddleware::new(secret.clone())) // 🔥 Protects all routes inside this scope
            .service(web::resource("/get-college-courses").route(web::get().to(get_college_course)))
            .service(web::resource("/get-school-courses").route(web::get().to(get_school_course)))
            .service(web::resource("/get-particular-course").route(web::get().to(get_particular_course)))
    );
}
