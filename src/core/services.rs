use std::time::{SystemTime, UNIX_EPOCH};

use polars::prelude::*;
use rand::Rng;
use serde_json::{json, Value};
use time::macros::{datetime, format_description};
use time::OffsetDateTime;

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

pub fn get_prices(ticker: &str) -> Value {
    let res = ureq::get(&format!(
        "https://query2.finance.yahoo.com/v8/finance/chart/{ticker}"
    ))
    .set("User-Agent", USER_AGENT)
    .query_pairs(vec![
        ("interval", "1d"),
        ("events", "capitalGain|div|split"),
        (
            "period1",
            &datetime!(2023-01-01 00:00:00)
                .assume_utc()
                .unix_timestamp()
                .to_string(),
        ),
        (
            "period2",
            &SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
        ),
    ])
    .call()
    .unwrap();

    // https://www.petergirnus.com/blog/rust-reqwest-http-get-json
    let mut data: serde_json::Value = res.into_json().unwrap();
    let format = format_description!("[year]-[month]-[day]");
    data["chart"]["result"][0]["timestamp"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .for_each(|x| {
            *x = serde_json::to_value(
                OffsetDateTime::from_unix_timestamp(x.as_i64().unwrap())
                    .unwrap()
                    .format(&format)
                    .unwrap(),
            )
            .unwrap()
        });
    data["chart"]["result"][0].clone()
}
