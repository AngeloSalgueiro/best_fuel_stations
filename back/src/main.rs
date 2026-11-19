use crate::data::database::{db_init};



pub mod data;

#[tokio::main]
async fn main() {
    //let _ = stations_fetcher().await;
    //
    // println!("{:?}", fetch_stations_data().await);
    let _ = db_init().await;
}
