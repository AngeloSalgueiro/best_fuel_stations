use best_fuel_stations::data::database::{db_check};
use best_fuel_stations::entities::coordinates::Coordinates;
use best_fuel_stations::models::stations_dao::StationDao;

#[tokio::main]
async fn main() {
    let conn = db_check().await.unwrap();
    let station_dao = StationDao::new(conn);

    let coord = Coordinates::new(47.320306, -1.910185);

    station_dao.get_nearest_stations(coord, 3).unwrap();
}
