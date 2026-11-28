use best_fuel_stations::data::database::db_check;
use best_fuel_stations::models::station_dao::StationDao;
use best_fuel_stations::routes::router::router;
use dotenv::dotenv;
use reqwest::{Method, header};
use tower_http::cors::{Any, CorsLayer};

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

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    let router = router(station_dao).layer(cors);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server listening at port 3000");
    axum::serve(listener, router).await.unwrap();
}
