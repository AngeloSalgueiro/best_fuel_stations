use reqwest::Error;
use serde_json::Value;
use std::env;

use crate::entities::station::Station;

// Fetch data from an API
async fn fetch_data(url: &str) -> Result<Value, Error> {
    let url = format!("{url}");

    let response = reqwest::get(&url).await?.error_for_status().unwrap();

    let data: Value = response.json().await?;

    Ok(data)
}

// Fetch all fuel stations from the french gouvernement api
pub async fn fetch_stations_data() -> Result<Vec<Station>, Box<dyn std::error::Error>> {
    let url = env::var("GOUV_API").expect("GOUV_API is not set");

    let data = fetch_data(&url).await.map_err(|e| {
        eprintln!("Error while fetching the data :{}", e);
        e
    })?;

    let stations: Vec<Station> = serde_json::from_value(data).map_err(|e| {
        eprintln!("{}", e);
        e
    })?;

    Ok(stations)
}
