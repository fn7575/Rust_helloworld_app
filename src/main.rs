use std::net::SocketAddr;

use axum::{handler::HandlerWithoutStateExt, response::Html, routing::get, Router};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .nest_service(
            "/media",
            ServeDir::new("assets").not_found_service(fallback_handler.into_service()),
        )
        .fallback(fallback_handler);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8888));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn fallback_handler() -> Html<&'static str> {
    Html("<h1>Nothing to see here!</h1>")
}
