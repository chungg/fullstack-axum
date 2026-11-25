mod api;
mod core;
mod settings;
mod views;

#[cfg(test)]
mod tests;

use std::sync::Arc;
use std::time::Duration;

use axum::{extract::State, response::IntoResponse, routing::get, Router};
use minijinja::{path_loader, Environment};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_scalar::{Scalar, Servable};

use crate::api::v1;
use crate::settings::Settings;
use crate::views::pages;

pub struct AppState {
    pub db: Pool<Postgres>,
    pub jinja: Environment<'static>,
    pub settings: Settings,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app().await).await.unwrap();
}

#[derive(OpenApi)]
#[openapi(info(title = "Axum API", version = "1.0.0"))]
struct ApiDoc;

async fn app() -> Router {
    // setup jinja
    let mut minijinja = Environment::new();
    minijinja.set_loader(path_loader("src/templates"));

    let settings = Settings::new().unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&settings.database_uri)
        .await
        .expect("Error creating db pool");

    // application routes
    // api routes that are captured in openapi
    let (api_router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(healthcheck))
        .routes(routes!(v1::routes::random_data))
        .routes(routes!(v1::routes::sales_data))
        .routes(routes!(v1::routes::price_data))
        .split_for_parts();

    // routes for ui
    let app_router: Router<Arc<AppState>> = Router::new()
        .route("/hx/v1/data/deaths", get(v1::hx::death_data))
        .route("/hx/v1/data/random", get(v1::hx::random_data))
        .route("/hx/v1/data/sales", get(v1::hx::sales_data))
        .route("/hx/v1/market/prices", get(v1::hx::price_data))
        .route("/", get(pages::index))
        .route("/analytics", get(pages::analytics))
        .route("/yahoo", get(pages::yahoo))
        .nest_service("/static/js", ServeDir::new("src/static/js"));

    api_router
        .merge(app_router)
        .merge(Scalar::with_url("/scalar", api))
        .with_state(Arc::new(AppState {
            db: pool,
            jinja: minijinja,
            settings,
        }))
        .layer(TraceLayer::new_for_http())
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service and database are healthy", body = String, content_type = "text/plain"),
        (status = 500, description = "Database healthcheck failed")
    )
)]
async fn healthcheck(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let _ = sqlx::query("select 1").fetch_one(&state.db).await.unwrap();
    "¯\\_(ツ)_/¯"
}
