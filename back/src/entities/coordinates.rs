pub struct Coordinates {
    pub latitude: f32,
    pub longitude: f32,
}

impl Coordinates {
    pub fn new(latitude: f32, longitude: f32) -> Self {
        if latitude < -90.0 || latitude > 90.0 {
            panic!("Error: Invalid latitude");
        }
        
        if longitude < -180.0 || longitude > 180.0 {
            panic!("Error: Invalid longitude");
        }

        Self {
            latitude,
            longitude,
        }
    }
}
