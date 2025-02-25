use actix_web::{get, web, HttpResponse, Responder};

#[get("/courses")]
async fn get_courses() -> impl Responder {
    HttpResponse::Ok().body("Course List")
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_courses);
}
