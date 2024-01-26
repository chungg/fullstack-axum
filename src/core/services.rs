use chrono::{NaiveDateTime, Utc};
use polars::prelude::*;
use rand::Rng;
use reqwest;
use serde_json::{json, Value};

pub fn random_data() -> Value {
    let mut rng = rand::thread_rng();
    let data = [(); 6].map(|_| rng.gen_range(0..100));
    let label = format!("blah{}", rng.gen_range(0..10000));
    json!({ "datasets": [{"data": data, "label": label}]})
}

pub fn sales_data() -> Value {
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

pub fn death_data() -> DataFrame {
    LazyCsvReader::new("src/static/data/can-deaths.csv")
        .has_header(true)
        .finish()
        .unwrap()
        .collect()
        .unwrap()
}

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_1) \
    AppleWebKit/537.36 (KHTML, like Gecko) \
    Chrome/39.0.2171.95 Safari/537.36";

pub async fn get_prices(ticker: &str) -> Value {
    let client = reqwest::Client::new();
    let res = client
        .get(format!(
            "https://query2.finance.yahoo.com/v8/finance/chart/{}",
            ticker
        ))
        .header("User-Agent", USER_AGENT)
        .query(&[
            ("interval", "1d"),
            ("events", "capitalGain|div|split"),
            (
                "period1",
                &NaiveDateTime::parse_from_str("2023-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap()
                    .timestamp()
                    .to_string(),
            ),
            ("period2", &Utc::now().timestamp().to_string()),
        ])
        .send()
        .await
        .unwrap();
    // https://www.petergirnus.com/blog/rust-reqwest-http-get-json
    let mut data: serde_json::Value = res.json().await.unwrap();
    data["chart"]["result"][0]["timestamp"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .for_each(|x| {
            *x = serde_json::to_value(
                NaiveDateTime::from_timestamp_opt(x.as_i64().unwrap(), 0)
                    .unwrap()
                    .format("%Y-%m-%d")
                    .to_string(),
            )
            .unwrap()
        });
    &data["chart"]["result"][0]
}
