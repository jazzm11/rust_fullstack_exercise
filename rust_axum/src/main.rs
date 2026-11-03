mod controllers;
mod routes;

use routes::spider_routes::app_routes;

#[tokio::main]
async fn main() {
    // Initialize the application routes
    let app = app_routes();
    // Bind the server to the specified address and port
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // Print a message indicating that the server is running
    println!("Server running at http://localhost:3000");
    // Start serving the application using the listener and the defined routes
    axum::serve(listener, app).await.unwrap();
}

