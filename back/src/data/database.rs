use crate::data::stations_fetcher::fetch_stations_data;
use crate::entities::station::Station;
use chrono::{DateTime, NaiveDateTime, Utc};
use rusqlite::{Connection, Error, Result};

// Initialize the database
pub async fn db_init() -> Result<Connection, Error> {
    let conn = db_connect()?;

    db_create_tables(&conn)?;

    db_stations_clear(&conn)?;

    let stations: Vec<Station> = fetch_stations_data().await.unwrap();

    let conn = db_insert_stations(conn, stations)?;

    Ok(conn)
}

// Check if the data in the db are still up to date
pub async fn db_check() -> Result<Connection, Error> {
    let conn = db_connect()?;
    let today = Utc::now();

    let last_update_string: Result<String> = conn.query_row(
        "SELECT value FROM app_metadata WHERE key='last_update'",
        [],
        |row| row.get(0),
    );

    let last_update = match last_update_string {
        Ok(value) => value,
        Err(_) => {
            return Ok(db_init().await?);
        }
    };

    let naive =
        NaiveDateTime::parse_from_str(last_update.as_str(), "%Y-%m-%d %H:%M:%S%.f UTC").unwrap();

    let last_update: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive, Utc);

    let diff = today.signed_duration_since(last_update);

    if diff.num_minutes() >= 5 {
        println!("Last_update was too old, resetting database");

        return Ok(db_init().await?);
    };

    Ok(conn)
}

// Connect to the db and return a Connection
pub fn db_connect() -> Result<Connection, Error> {
    let conn = Connection::open("database.db")?;

    return Ok(conn);
}

// Clear the data related to the stations from the db
fn db_stations_clear(conn: &Connection) -> Result<(), Error> {
    conn.execute("DELETE FROM fuels", ())?;
    conn.execute("DELETE FROM stations", ())?;
    conn.execute("DELETE FROM app_metadata WHERE key = 'last_update'", ())?;

    println!("Cleared old data.");

    Ok(())
}

// Create the tables if they don't exists
fn db_create_tables(conn: &Connection) -> Result<(), Error> {
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

    CREATE TABLE IF NOT EXISTS app_metadata (
        key TEXT PRIMARY KEY,
        value TEXT NOT NULL
    );
    ",
    )
    .expect("Error while creating the tables");

    println!("Tables created successfully.");

    Ok(())
}

// Insert the data related to the stations in db
fn db_insert_stations(mut conn: Connection, stations: Vec<Station>) -> Result<Connection, Error> {
    println!("Inserting stations into database...");
    let tx = conn.transaction()?;

    {
        let mut last_update_stmt = tx.prepare(
            "INSERT INTO app_metadata (key, value)
            VALUES ('last_update', ?1)",
        )?;

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

        let today = Utc::now().to_string();
        last_update_stmt.execute((today,))?;
    }

    tx.commit()?;

    println!("Stations inserted succesfully!");

    Ok(conn)
}
