use axum::{Router, routing::get};

async fn hello_world() -> &'static str {
    "Hello, World!"
}

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
    // Add more routes here as needed
}
