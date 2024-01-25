use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use polars::prelude::*;
use serde_json::json;
use uuid::Uuid;

use crate::core::services;

pub async fn random_data(headers: HeaderMap, req: Request) -> impl IntoResponse {
    let data = services::random_data().to_string();
    let data_id = Uuid::new_v4().to_string();
    let res_headers = [(
        "HX-Trigger-After-Swap",
        json!({"apiResponse": {"origin": headers.get("hx-current-url").unwrap().to_str().unwrap(),
                                "path": req.uri().to_string(), "dataId": data_id}})
        .to_string(),
    )];
    let resp = format!(
        r#"<script id="{}" type="application/json">{}</script>"#,
        data_id, data
    );
    (StatusCode::OK, res_headers, resp)
}

pub async fn sales_data(headers: HeaderMap, req: Request) -> impl IntoResponse {
    let data = services::sales_data().to_string();
    let data_id = Uuid::new_v4().to_string();
    let res_headers = [(
        "HX-Trigger-After-Swap",
        json!({"apiResponse": {"origin": headers.get("hx-current-url").unwrap().to_str().unwrap(),
                                "path": req.uri().to_string(), "dataId": data_id}})
        .to_string(),
    )];
    let resp = format!(
        r#"<script id="{}" type="application/json">{}</script>"#,
        data_id, data
    );
    (StatusCode::OK, res_headers, resp)
}

pub async fn death_data() -> String {
    let mut table = services::death_data();
    let col_props = table
        .get_column_names()
        .into_iter()
        .filter(|&x| x != "cause")
        .map(|x| {
            format!(
                "{{title:\"{x}\", field:\"{x}\", headerHozAlign:\"center\", hozAlign:\"right\"}}",
                x = x
            )
        })
        .collect::<Vec<String>>()
        .join(",");
    let mut buf = Vec::new();
    JsonWriter::new(&mut buf)
        .with_json_format(JsonFormat::Json)
        .finish(&mut table)
        .unwrap();
    format!(
        r###"
            <script>
              new Tabulator("#death-table", {{
                index: "cause",
                layout: "fitColumns",
                data: {},
                frozenRowsField: "cause",
                frozenRows: ["Total"],
                columnHeaderVertAlign: "bottom",
                columns: [
                  {{title: "cause", field: "cause", resizable: "header", frozen: true,}},
                  {{title: "year",
                   headerHozAlign: "center",
                   columns: [{}]
                  }},
                ]
              }});
            </script>
            "###,
        String::from_utf8(buf).unwrap(),
        col_props
    )
}
