use dotenv::dotenv;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Error;
use serde_json::Value;
use std::env;

use crate::entities::station::Station;

// Fetch data from an API
async fn fetch_data(raw_url: &str, limit: u16, offset: u64) -> Result<Value, Error> {
    let url = format!("{raw_url}?limit={limit}&offset={offset}");

    let response = reqwest::get(&url).await?.error_for_status()?;

    let data: Value = response.json().await?;

    Ok(data)
}

// Fetch all fuel stations from the french gouvernement api
pub async fn fetch_stations_data() -> Result<Vec<Station>, Box<dyn std::error::Error>> {
    dotenv().ok();
    let url = env::var("GOUV_API").expect("GOUV_API is not set");

    let limit = 100;

    let data = fetch_data(&url, limit, 0)
        .await
        .expect("Error while fetching the data");

    let total_count: u64 = data["total_count"]
        .as_u64()
        .expect("total_count is not an integer");

    let mut offset: u64 = limit as u64;

    let mut results: Vec<Station> = Vec::new();
    let stations: Vec<Station> = serde_json::from_value(data["results"].clone())?;

    results.extend(stations);

    println!("Fetching stations...");
    let bar = ProgressBar::new(total_count);
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .progress_chars("##-"),
    );

    bar.inc(limit as u64);

    while offset < total_count {
        let data = fetch_data(&url, limit, offset).await?;

        let stations: Vec<Station> = serde_json::from_value(data["results"].clone())?;

        results.extend(stations);
        offset += limit as u64;

        bar.inc(limit as u64);
    }

    bar.finish();

    Ok(results)
}
