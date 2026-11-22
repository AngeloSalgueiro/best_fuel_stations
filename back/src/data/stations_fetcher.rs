use dotenv::dotenv;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Error;
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Fuel {
    #[serde(rename = "@nom")]
    pub name: String,

    #[serde(rename = "@maj")]
    pub updated_at: String,

    #[serde(rename = "@valeur")]
    pub price: String,
}

#[derive(Debug, Deserialize)]
pub struct Station {
    pub id: i64,

    #[serde(deserialize_with = "string_to_f64")]
    pub latitude: Option<f64>,

    #[serde(deserialize_with = "string_to_f64")]
    pub longitude: Option<f64>,

    #[serde(rename = "ville")]
    pub city: Option<String>,

    #[serde(rename = "adresse")]
    pub adress: Option<String>,

    pub cp: Option<String>,

    #[serde(rename = "prix", default, deserialize_with = "string_to_fuels")]
    pub fuels: Vec<Fuel>,
}

fn string_to_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<String> = Option::deserialize(deserializer)?;

    match value {
        Some(s) => s.parse::<f64>().map(Some).map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

fn string_to_fuels<'de, D>(deserializer: D) -> Result<Vec<Fuel>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;

    let value = match value {
        serde_json::Value::String(s) => {
            serde_json::from_str(&s).map_err(serde::de::Error::custom)?
        }
        other => other,
    };

    match value {
        serde_json::Value::Array(_) => {
            serde_json::from_value(value).map_err(serde::de::Error::custom)
        }
        serde_json::Value::Object(_) => {
            let fuel: Fuel = serde_json::from_value(value).map_err(serde::de::Error::custom)?;

            Ok(vec![fuel])
        }
        serde_json::Value::Null => Ok(Vec::new()),
        _ => Err(serde::de::Error::custom(
            "Expected a fuel object, array, or JSON string",
        )),
    }
}

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
