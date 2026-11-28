use axum::{
    Json,
    extract::{Path, State},
};
use reqwest::StatusCode;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    entities::{coordinates::Coordinates, station::Station},
    models::station_dao::StationDao,
};

#[derive(Clone)]
pub struct AppState {
    pub station_dao: Arc<Mutex<StationDao>>,
}

pub async fn stations_sample(State(state): State<AppState>) -> (StatusCode, Json<Vec<Station>>) {
    let dao = state.station_dao.lock().await;

    let stations: Vec<Station> = dao
        .get_stations_sample(5)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .unwrap();

    (StatusCode::OK, Json(stations))
}

#[axum::debug_handler]
pub async fn nearest_stations(
    Path(nb_stations): Path<u8>,
    State(state): State<AppState>,
    Json(payload): Json<Coordinates>,
) -> (StatusCode, Json<Vec<Station>>) {
    let dao = state.station_dao.lock().await;

    let stations: Vec<Station> = dao
        .get_nearest_stations(payload, nb_stations)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .unwrap();

    (StatusCode::OK, Json(stations))
}
