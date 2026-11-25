use best_fuel_stations::data::database::db_check;
use best_fuel_stations::entities::coordinates::Coordinates;
use best_fuel_stations::models::stations_dao::StationDao;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let conn = db_check()
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            e
        })
        .unwrap();

    let station_dao = StationDao::new(conn);

    let coord = Coordinates::new(47.320306, -1.910185);

    let stations = station_dao
        .get_nearest_stations(coord, 3)
        .map_err(|e| {
            eprintln!("{}", e);
            e
        })
        .unwrap();

    for station in stations {
        println!(
            "{} Adress: {} {} {}, Coordinates: {}, {}",
            station.id,
            station.adress.unwrap(),
            station.city.unwrap(),
            station.cp.unwrap(),
            station.latitude.unwrap(),
            station.longitude.unwrap()
        )
    }
}
