use actix_web::web;
use crate::controllers::user_controller::{get_user, update_student};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/external/v1/user/get-user").route(web::get().to(get_user)))
       .service(web::resource("/external/v1/user/update-user").route(web::post().to(update_student)));
       // .service(web::resource("/auth/check-loggedin-user").route(web::get().to(check_logged_in_user))); // Uncomment when middleware is added
}
