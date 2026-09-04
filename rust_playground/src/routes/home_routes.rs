use actix_web::web;

use crate::controllers::home_controller;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(home_controller::home))
    );
}