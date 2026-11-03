use axum::{Router, routing::get};
use tower_http::services::ServeDir;

use crate::controllers::spider_controller::{facts, gallery, home};

pub fn app_routes() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/facts", get(facts))
        .route("/gallery", get(gallery))
        .nest_service("/static", ServeDir::new("public"))
}
