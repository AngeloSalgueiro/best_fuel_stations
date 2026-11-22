use rusqlite::{Connection, Error, Result, Statement};
use std::{env, fs};

use crate::data::stations_fetcher::{Station, fetch_stations_data};

pub async fn db_init() -> Result<Connection, Error> {
    let exe_dir = env::current_exe()
        .expect("Cannot get executable path")
        .parent()
        .expect("Cannot get executable directory")
        .to_path_buf();

    let db_path = exe_dir.join("database.db");

    println!("Clearing old database.");

    fs::remove_file(db_path.clone()).ok();

    println!("Initializing database.");

    let mut conn = Connection::open(db_path).expect("Error while opening the database");

    println!("Database created!");

    conn.execute_batch(
        "
    CREATE TABLE IF NOT EXISTS stations (
            id INTEGER PRIMARY KEY,
            latitude REAL,
            longitude REAL,
            city TEXT,
            adress TEXT,
            cp TEXT
        );

    CREATE TABLE IF NOT EXISTS fuels(
         id INTEGER PRIMARY KEY AUTOINCREMENT,
         fuel_type TEXT NOT NULL,
         price REAL NOT NULL,
         last_update TEXT,
         station_id INTEGER REFERENCES stations(id)
    );
    ",
    )
    .expect("Error while creating the tables");

    println!("Tables created successfully.");

    let stations: Vec<Station> = fetch_stations_data().await.unwrap();

    println!("Inserting stations into database...");

    let tx = conn.transaction()?;

    {
        let mut station_stmt = tx.prepare(
            "INSERT INTO stations (id, latitude, longitude, city, adress, cp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        )?;

        let mut fuel_stmt = tx.prepare(
            "INSERT INTO fuels (fuel_type, price, last_update, station_id)
             VALUES (?1, ?2, ?3, ?4)",
        )?;

        for station in stations {
            station_stmt.execute((
                station.id,
                station.latitude,
                station.longitude,
                station.city.as_deref(),
                station.adress.as_deref(),
                station.cp.as_deref(),
            ))?;

            for fuel in station.fuels {
                fuel_stmt.execute((fuel.name, fuel.price, fuel.updated_at, station.id))?;
            }
        }
    }

    tx.commit()?;

    println!("Stations inserted succesfully!");

    Ok(conn)
}

pub fn db_check() {}

pub fn db_connect() -> Result<Connection, Error> {
    let conn = Connection::open("database.db")?;

    return Ok(conn);
}

pub fn db_test() {
    let conn = Connection::open("database.db").expect("Error while opening database");

    let mut stmt: Statement = conn
        .prepare("SELECT * FROM stations where city = 'Savenay'")
        .expect("Error while preparing statement");

    let station_iter = stmt
        .query_map([], |row| {
            Ok(Station {
                id: row.get(0)?,
                latitude: row.get(1)?,
                longitude: row.get(2)?,
                city: row.get(3)?,
                adress: row.get(4)?,
                cp: row.get(5)?,
                fuels: vec![],
            })
        })
        .expect("Error while executing statement");

    for station in station_iter {
        println!("{:?}", station)
    }
}
