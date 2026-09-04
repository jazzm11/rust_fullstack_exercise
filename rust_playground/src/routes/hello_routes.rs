use actix_web::web;

use crate::controllers::hello_controller;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/hello")
            .route(web::get().to(hello_controller::hello))
    );
}