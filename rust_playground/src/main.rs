use actix_files::Files;
use actix_web::{App, HttpServer};

mod routes;
mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes::home_routes::config)
            .configure(routes::hello_routes::config)
            .service(Files::new("/", "./static")) // Serve static files from the "static" directory
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}