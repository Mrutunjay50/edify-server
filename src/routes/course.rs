use actix_web::web;

use crate::controllers::course_controller::{get_college_course, get_particular_course, get_school_course};


pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/course/get-college-courses").route(web::get().to(get_college_course)))
       .service(web::resource("/course/get-school-courses").route(web::get().to(get_school_course)))
       .service(web::resource("/course/get-particular-course").route(web::get().to(get_particular_course)));
}
