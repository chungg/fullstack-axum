use axum::{extract::Query, response::Json};
use serde_json::Value;

use crate::api::v1::hx;
use crate::core::services;

pub async fn random_data() -> Json<Value> {
    Json(services::random_data())
}

pub async fn sales_data() -> Json<Value> {
    Json(services::sales_data())
}

pub async fn price_data(Query(params): Query<hx::PriceParams>) -> Json<Value> {
    Json(services::get_prices(&params.ticker).await)
}
