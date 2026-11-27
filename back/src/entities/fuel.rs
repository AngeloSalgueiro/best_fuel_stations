use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Fuel {
    #[serde(rename = "@nom")]
    pub name: String,

    #[serde(rename = "@maj")]
    pub updated_at: String,

    #[serde(rename = "@valeur")]
    pub price: String,
}
