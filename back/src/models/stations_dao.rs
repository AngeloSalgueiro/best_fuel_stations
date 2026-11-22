use rusqlite::{Connection, Error, Result, Statement};
use crate::data::stations_fetcher::{Station};


pub struct StationDao{
    conn: Connexion
}

impl StationDao{
    pub fn getAll() -> Vec<Station>;
    pub fn getOne() -> Station;
    pub fn getNearestStation() -> Station;
    pub fn getNearestStations() -> Vec<Station>;
}
