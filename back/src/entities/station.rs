use crate::entities::fuel::Fuel;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct Station {
    pub id: i64,

    #[serde(deserialize_with = "string_to_coord")]
    pub latitude: Option<f64>,

    #[serde(deserialize_with = "string_to_coord")]
    pub longitude: Option<f64>,

    #[serde(rename = "ville")]
    pub city: Option<String>,

    #[serde(rename = "adresse")]
    pub adress: Option<String>,

    pub cp: Option<String>,

    #[serde(rename = "prix", default, deserialize_with = "string_to_fuels")]
    pub fuels: Vec<Fuel>,
}

fn string_to_coord<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<String> = Option::deserialize(deserializer)?;

    match value {
        Some(s) => s
            .parse::<f64>()
            .map(|v| Some(v / 100_000.0))
            .map_err(serde::de::Error::custom),
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
