use axum::{
    Router,
    routing::{get, post},
};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    handlers::stations_handler::{AppState, nearest_stations, stations_sample},
    models::station_dao::StationDao,
};

pub fn router(station_dao: StationDao) -> Router {
    let url = env::var("API_URL").expect("GOUV_API is not set");

    let state = AppState {
        station_dao: Arc::new(Mutex::new(station_dao)),
    };

    Router::new()
        .route(&format!("{}/", url), get(stations_sample))
        .route(&format!("{}/nearest", url), post(nearest_stations))
        .with_state(state)
}
