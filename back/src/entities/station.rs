use crate::entities::fuel::Fuel;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct Station {
    pub id: i64,

    #[serde(deserialize_with = "deserialize_coord")]
    pub latitude: Option<f64>,

    #[serde(deserialize_with = "deserialize_coord")]
    pub longitude: Option<f64>,

    #[serde(rename = "Ville")]
    pub city: Option<String>,

    #[serde(rename = "Adresse")]
    pub adress: Option<String>,

    #[serde(rename = "Code postal")]
    pub cp: Option<String>,

    #[serde(default, rename = "prix")]
    pub fuels: Vec<Fuel>,
}

fn deserialize_coord<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<f64>::deserialize(deserializer)?;
    Ok(value.map(|v| v / 100_000.0))
}
