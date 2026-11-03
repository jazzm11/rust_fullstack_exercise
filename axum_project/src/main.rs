use axum::{routing::get, Router, Json};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    title: String,
    message: String,
}

async fn hello_world() -> Json<Response> {
    let response = Response {
        title: "Hello, World!".to_string(),
        message: "Welcome to the Axum server.".to_string(),
    };
    Json(response)
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:3000";

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Server running on {addr:?}");

    axum::serve(listener, router()).await.unwrap();
}

// ROUTER
// Defining the router function that returns a Router instance
fn router() -> Router {
    // Create a new Router instance and merge the application routes into it
    Router::new()
    .route("/api/hello_world", get(hello_world))
}