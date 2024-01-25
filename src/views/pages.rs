use std::sync::Arc;

use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use minijinja::context;
use serde_json::json;

use crate::AppState;

pub async fn analytics(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let out = state
        .jinja
        .get_template("analytics.html")
        .unwrap()
        .render(context!())
        .unwrap();
    let res_headers = [(
        "HX-Trigger-After-Settle",
        json!({"initPage": {"path": "/analytics"}}).to_string(),
    )];
    (res_headers, Html(out))
}
