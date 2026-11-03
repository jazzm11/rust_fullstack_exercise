use axum::response::Html;

const HOME_HTML: &str = include_str!("../views/home.html");
const FACTS_HTML: &str = include_str!("../views/facts.html");
const GALLERY_HTML: &str = include_str!("../views/gallery.html");

pub async fn home() -> Html<&'static str> {
    Html(HOME_HTML)
}

pub async fn facts() -> Html<&'static str> {
    Html(FACTS_HTML)
}

pub async fn gallery() -> Html<&'static str> {
    Html(GALLERY_HTML)
}
