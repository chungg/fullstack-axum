use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Json},
    routing::get,
    Router,
};
use minijinja::{context, path_loader, Environment};
use polars::prelude::*;
use rand::Rng;
use serde_json::{json, Value};
use tower_http::services::ServeDir;
use uuid::Uuid;

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
        .route("/api/v1/data/random", get(api_random_data))
        .route("/api/v1/data/sales", get(api_sales_data))
        .route("/hx/v1/data/deaths", get(hx_death_data))
        .route("/hx/v1/data/random", get(hx_random_data))
        .route("/hx/v1/data/sales", get(hx_sales_data))
        .route("/analytics", get(view_analytics))
        .nest_service("/static/js", ServeDir::new("src/static/js"))
        .with_state(Arc::new(AppState { jinja: minijinja }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn random_data() -> Value {
    let mut rng = rand::thread_rng();
    let data = [(); 6].map(|_| rng.gen_range(0..100));
    let label = format!("blah{}", rng.gen_range(0..10000));
    json!({ "datasets": [{"data": data, "label": label}]})
}

async fn api_random_data() -> Json<Value> {
    Json(random_data().await)
}

async fn hx_random_data(headers: HeaderMap, req: Request) -> impl IntoResponse {
    let data = random_data().await.to_string();
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

async fn sales_data() -> Value {
    let options = StrptimeOptions {
        format: Some("%m/%d/%Y %I:%M:%S %P".into()),
        ..Default::default()
    };
    let table = LazyCsvReader::new("src/static/data/Monthly_Transportation_Statistics.csv")
        .has_header(true)
        .finish()
        .unwrap()
        .filter(col("Auto sales").is_not_null())
        .select([
            col("Date").str().to_date(options),
            col("Auto sales").cast(DataType::Int32),
            col("Light truck sales").cast(DataType::Int32),
        ])
        .collect()
        .unwrap();
    json!({"datasets": [
        {
         "label": "auto",
         "data": &table["Auto sales"].i32().unwrap()
             .into_no_null_iter().collect::<Vec<i32>>()
        },
        {
         "label": "truck",
         "data": &table["Light truck sales"].i32().unwrap()
             .into_no_null_iter().collect::<Vec<i32>>()
        }],
        "labels": &table["Date"].strftime("%Y-%m-%d").unwrap().str().unwrap()
            .into_no_null_iter().collect::<Vec<&str>>()
    })
}

async fn api_sales_data() -> Json<Value> {
    Json(sales_data().await)
}

async fn hx_sales_data(headers: HeaderMap, req: Request) -> impl IntoResponse {
    let data = sales_data().await.to_string();
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

async fn hx_death_data() -> String {
    let mut table = LazyCsvReader::new("src/static/data/can-deaths.csv")
        .has_header(true)
        .finish()
        .unwrap()
        .collect()
        .unwrap();
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

async fn view_analytics(State(state): State<Arc<AppState>>) -> impl IntoResponse {
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
