use data::stations_fetcher::stations_fetcher;

pub mod data;

#[tokio::main]
async fn main() {
    let _ = stations_fetcher().await;
}
