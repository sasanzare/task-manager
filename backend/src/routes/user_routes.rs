use actix_web::web;
use crate::handlers::user_handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/auth/register")
            .route(web::post().to(user_handler::register))
    )
    .service(
        web::resource("/auth/login")
            .route(web::post().to(user_handler::login))
    )
    .service(
        web::resource("/users/{id}")
            .route(web::get().to(user_handler::get_user)),
    );
}