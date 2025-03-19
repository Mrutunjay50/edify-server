pub mod auth;
pub mod course;
pub mod user;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    auth::auth_routes(cfg);
    course::course_routes(cfg);
    user::user_routes(cfg);
}
