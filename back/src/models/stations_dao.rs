use rusqlite::{Connection, Result};

use crate::entities::coordinates::Coordinates;
use crate::entities::station::Station;

pub struct StationDao {
    conn: Connection,
}

impl StationDao {
    pub fn new(conn: Connection) -> Self {
        return Self { conn };
    }

    //     pub fn getAll() -> Vec<Station>{
    //         let mut stmt = conn
    //             .prepare("SELECT * FROM stations")
    //             .expect("Error while preparing statement");
    //
    //         return ;
    //     };
    // pub fn getOne() -> Station;
    pub fn get_nearest_station(&self, coord: Coordinates) -> Result<Station> {
        let mut stmt = self.conn.prepare(
            "
            SELECT *,
                6371000 * 2 * ASIN(
                    SQRT(
                        POWER(SIN(RADIANS(latitude - ?1) / 2), 2) +
                        COS(RADIANS(?1)) *
                        COS(RADIANS(latitude)) *
                        POWER(SIN(RADIANS(longitude - ?2) / 2), 2)
                    )
                ) AS distance
            FROM stations
            ORDER BY distance ASC
            LIMIT 1;
        ",
        )?;

        let station_iter = stmt.query_map([coord.latitude, coord.longitude], |row| {
            Ok(Station {
                id: row.get(0)?,
                latitude: row.get(1)?,
                longitude: row.get(2)?,
                city: row.get(3)?,
                adress: row.get(4)?,
                cp: row.get(5)?,
                fuels: vec![],
            })
        })?;

        for station in station_iter {
            return Ok(station?);
        }

        panic!("No station found");
    }

    pub fn get_nearest_stations(
        &self,
        coord: Coordinates,
        nb_stations: u8,
    ) -> Result<Vec<Station>> {
        let mut stmt = self.conn.prepare(
            "
        SELECT *,
            6371000 * 2 * ASIN(
                SQRT(
                    POWER(SIN(RADIANS(latitude - ?1) / 2), 2) +
                    COS(RADIANS(?1)) *
                    COS(RADIANS(latitude)) *
                    POWER(SIN(RADIANS(longitude - ?2) / 2), 2)
                )
            ) AS distance
        FROM stations
        ORDER BY distance ASC
        LIMIT ?3;
        ",
        )?;

        let station_iter = stmt.query_map(
            [coord.latitude, coord.longitude, nb_stations as f32],
            |row| {
                Ok(Station {
                    id: row.get(0)?,
                    latitude: row.get(1)?,
                    longitude: row.get(2)?,
                    city: row.get(3)?,
                    adress: row.get(4)?,
                    cp: row.get(5)?,
                    fuels: vec![],
                })
            },
        )?;

        let stations = station_iter.collect::<rusqlite::Result<Vec<Station>>>()?;

        return Ok(stations);
    }
}
