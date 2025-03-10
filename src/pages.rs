use axum::{response::Html, routing::get, Router};

async fn home() -> Html<&'static str> {
    println!("[GET] /");
    Html("Hello, world!")
}

async fn about() -> Html<&'static str> {
    println!("[GET] /about");
    Html("about")
}

async fn contact() -> Html<&'static str> {
    println!("[GET] /contact");
    Html("contact")
}


pub async fn create_pages_router() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/about", get(about))
        .route("/contact", get(contact))
}