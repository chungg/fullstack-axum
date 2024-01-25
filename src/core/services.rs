use polars::prelude::*;
use rand::Rng;
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
