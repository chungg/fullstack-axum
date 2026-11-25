use axum::{extract::Query, response::Json};
use serde_json::Value;

use crate::api::v1::hx;
use crate::core::services;

#[utoipa::path(
    get,
    path = "/api/v1/data/random",
    responses(
        (status = 200, description = "Random data payload", body = Value)
    )
)]
pub async fn random_data() -> Json<Value> {
    Json(services::random_data())
}

#[utoipa::path(
    get,
    path = "/api/v1/data/sales",
    responses(
        (status = 200, description = "Sales data payload", body = Value)
    )
)]
pub async fn sales_data() -> Json<Value> {
    Json(services::sales_data())
}

#[utoipa::path(
    get,
    path = "/api/v1/market/prices",
    params(hx::PriceParams),
    responses(
        (status = 200, description = "Historical price data for a ticker", body = Value)
    )
)]
pub async fn price_data(Query(params): Query<hx::PriceParams>) -> Json<Value> {
    Json(services::get_prices(&params.ticker))
}
