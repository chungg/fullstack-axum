use jiff::{tz::TimeZone, Timestamp};
use polars::prelude::*;
use rand::RngExt;
use serde_json::{json, Value};

pub fn random_data() -> Value {
    let mut rng = rand::rng();
    let data = [(); 6].map(|_| rng.random_range(0..100));
    let label = format!("blah{}", rng.random_range(0..10000));
    json!({ "datasets": [{"data": data, "label": label}]})
}

pub fn sales_data() -> Value {
    let options = StrptimeOptions {
        format: Some("%m/%d/%Y %I:%M:%S %P".into()),
        ..Default::default()
    };
    let table = LazyCsvReader::new(PlRefPath::new(
        "src/static/data/Monthly_Transportation_Statistics.csv",
    ))
    .with_has_header(true)
    .finish()
    .unwrap()
    .filter(col("Auto sales").is_not_null())
    .select([
        col("Date")
            .cast(DataType::String)
            .str()
            .to_date(options)
            .dt()
            .strftime("%Y-%m-%d"),
        col("Auto sales").cast(DataType::Int32),
        col("Light truck sales").cast(DataType::Int32),
    ])
    .drop_nulls(None)
    .collect()
    .unwrap();

    let auto_sales: Vec<i32> = table["Auto sales"]
        .as_materialized_series()
        .i32()
        .unwrap()
        .into_no_null_iter()
        .collect();

    let truck_sales: Vec<i32> = table["Light truck sales"]
        .as_materialized_series()
        .i32()
        .unwrap()
        .into_no_null_iter()
        .collect();

    let labels: Vec<String> = table["Date"]
        .as_materialized_series()
        .str()
        .unwrap()
        .iter()
        .flatten()
        .map(str::to_owned)
        .collect();

    json!({
        "datasets": [
            {
                "label": "auto",
                "data": auto_sales
            },
            {
                "label": "truck",
                "data": truck_sales
            }
        ],
        "labels": labels
    })
}

pub fn death_data() -> DataFrame {
    LazyCsvReader::new(PlRefPath::new("src/static/data/can-deaths.csv"))
        .with_has_header(true)
        .finish()
        .unwrap()
        .collect()
        .unwrap()
}

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:160.0) \
    Gecko/20100101 \
    Firefox/160.0 ";

pub fn get_prices(ticker: &str) -> Value {
    let period1 = jiff::civil::date(2023, 1, 1)
        .at(0, 0, 0, 0)
        .to_zoned_datetime(TimeZone::UTC)
        .unwrap()
        .timestamp()
        .as_second()
        .to_string();
    let period2 = Timestamp::now().as_second().to_string();

    let query_params = vec![
        ("interval", "1d".to_string()),
        ("events", "capitalGain|div|split".to_string()),
        ("period1", period1),
        ("period2", period2),
    ];

    let mut res = ureq::get(&format!(
        "https://query2.finance.yahoo.com/v8/finance/chart/{ticker}"
    ))
    .header("User-Agent", USER_AGENT)
    .query_pairs(query_params)
    .call()
    .unwrap();

    // https://www.petergirnus.com/blog/rust-reqwest-http-get-json
    let mut data: serde_json::Value = res.body_mut().read_json().unwrap();

    if let Some(timestamps) = data["chart"]["result"][0]["timestamp"].as_array_mut() {
        for ts in timestamps.iter_mut() {
            if let Some(secs) = ts.as_i64() {
                let formatted = Timestamp::from_second(secs)
                    .unwrap()
                    .strftime("%Y-%m-%d")
                    .to_string();
                *ts = serde_json::Value::String(formatted);
            }
        }
    }

    data["chart"]["result"][0].clone()
}
