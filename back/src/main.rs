use best_fuel_stations::data::database::db_check;
use best_fuel_stations::models::station_dao::StationDao;
use best_fuel_stations::routes::router::router;
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

    let router = router(station_dao);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server listening at port 3000");
    axum::serve(listener, router).await.unwrap();
}
