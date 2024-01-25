mod api;
mod core;
mod views;

use axum::{
    routing::get,
    Router,
};
use minijinja::{path_loader, Environment};
use tower_http::services::ServeDir;

use std::sync::Arc;

use crate::api::v1;
use crate::views::pages;

pub struct AppState {
    pub jinja: Environment<'static>,
}

#[tokio::main]
async fn main() {
    //// setup jinja
    let mut minijinja = Environment::new();
    minijinja.set_loader(path_loader("src/templates"));

    // application routes
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/v1/data/random", get(v1::routes::random_data))
        .route("/api/v1/data/sales", get(v1::routes::sales_data))
        .route("/hx/v1/data/deaths", get(v1::hx::death_data))
        .route("/hx/v1/data/random", get(v1::hx::random_data))
        .route("/hx/v1/data/sales", get(v1::hx::sales_data))
        .route("/analytics", get(pages::analytics))
        .nest_service("/static/js", ServeDir::new("src/static/js"))
        .with_state(Arc::new(AppState { jinja: minijinja }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
