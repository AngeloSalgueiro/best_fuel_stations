use dotenv::dotenv;
use reqwest::Error;
use serde_json::Value;
use std::env;

// Fetch data from an API
async fn fetch_data(raw_url: &str, limit: u16, offset: u64) -> Result<Value, Error> {
    let url = format!("{raw_url}?limit={limit}&offset={offset}");

    let response = reqwest::get(&url).await?.error_for_status()?;

    let data: Value = response.json().await?;

    Ok(data)
}

// Fetch all fuel stations from the french gouvernement api
async fn fetch_stations_data(limit: u16) -> Vec<Value> {
    dotenv().ok();
    let url = env::var("GOUV_API").expect("GOUV_API is not set");

    let data = fetch_data(&url, limit, 0)
        .await
        .expect("Error while fetching the data");

    let total_count: u64 = data["total_count"]
        .as_u64()
        .expect("total_count is not an integer");

    let mut offset: u64 = limit as u64;

    let mut results: Vec<Value> = Vec::new();
    let result = data["results"].as_array().expect("Invalid results");

    results.extend(result.iter().cloned());

    println!("Fetched {} elements", offset);

    while offset < total_count {
        let data = fetch_data(&url, limit, offset)
            .await
            .expect("Error while fetching the data");

        let result = data["results"].as_array().expect("Invalid results");

        results.extend(result.iter().cloned());
        offset += limit as u64;
        println!("Fetched {} elements", offset);
    }

    results
}

pub async fn stations_fetcher() {
    let stations = fetch_stations_data(100).await;

    println!("{:?}", stations);
}
