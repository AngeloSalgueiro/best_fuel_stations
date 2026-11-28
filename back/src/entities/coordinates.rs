use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize)]
pub struct Coordinates {
    latitude: f32,
    longitude: f32,
}

impl Coordinates {
    pub fn new(latitude: f32, longitude: f32) -> Result<Self, String> {
        if !(-90.0..=90.0).contains(&latitude) {
            return Err("Error: Invalid latitude".into());
        }

        if !(-180.0..=180.0).contains(&longitude) {
            return Err("Error: Invalid longitude".into());
        }

        Ok(Self {
            latitude,
            longitude,
        })
    }

    pub fn get_latitude(&self) -> f32 {
        return self.latitude;
    }

    pub fn get_longitude(&self) -> f32 {
        return self.longitude;
    }
}

impl<'de> Deserialize<'de> for Coordinates {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct CoordinatesData {
            latitude: f32,
            longitude: f32,
        }

        let data = CoordinatesData::deserialize(deserializer)?;

        Ok(Coordinates::new(data.latitude, data.longitude).unwrap())
    }
}
