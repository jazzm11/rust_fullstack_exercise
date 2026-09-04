use actix_web::{HttpResponse, Responder};
use actix_files::NamedFile;

pub async fn home() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}

