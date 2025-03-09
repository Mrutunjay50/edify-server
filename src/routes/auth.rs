use actix_web::web;
use crate::controllers::auth_controller::{ login_user, register_user};

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/external/v1/auth/register-user").route(web::post().to(register_user)))
       .service(web::resource("/external/v1/auth/login-user").route(web::post().to(login_user)));
       // .service(web::resource("/auth/check-loggedin-user").route(web::get().to(check_logged_in_user))); // Uncomment when middleware is added
}
