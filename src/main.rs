mod api;
mod core;
mod views;

#[cfg(test)]
mod tests;

use std::sync::Arc;

use axum::{routing::get, Router};
use minijinja::{path_loader, Environment};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::api::v1;
use crate::views::pages;

pub struct AppState {
    pub jinja: Environment<'static>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "fullstack_axum=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app()).await.unwrap();
}

fn app() -> Router {
    // setup jinja
    let mut minijinja = Environment::new();
    minijinja.set_loader(path_loader("src/templates"));

    // application routes
    Router::new()
        .route("/health", get(|| async { "¯\\_(ツ)_/¯" }))
        .route("/api/v1/data/random", get(v1::routes::random_data))
        .route("/api/v1/data/sales", get(v1::routes::sales_data))
        .route("/api/v1/market/prices", get(v1::routes::price_data))
        .route("/hx/v1/data/deaths", get(v1::hx::death_data))
        .route("/hx/v1/data/random", get(v1::hx::random_data))
        .route("/hx/v1/data/sales", get(v1::hx::sales_data))
        .route("/hx/v1/market/prices", get(v1::hx::price_data))
        .route("/", get(pages::index))
        .route("/analytics", get(pages::analytics))
        .route("/yahoo", get(pages::yahoo))
        .nest_service("/static/js", ServeDir::new("src/static/js"))
        .with_state(Arc::new(AppState { jinja: minijinja }))
        .layer(TraceLayer::new_for_http())
}
