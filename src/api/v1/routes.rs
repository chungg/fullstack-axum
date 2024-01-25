use axum::response::Json;
use serde_json::Value;

use crate::core::services;

pub async fn random_data() -> Json<Value> {
    Json(services::random_data())
}

pub async fn sales_data() -> Json<Value> {
    Json(services::sales_data())
}
