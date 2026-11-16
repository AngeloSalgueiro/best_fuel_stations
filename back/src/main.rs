use models::temp::temp;

pub mod models;

#[tokio::main]
async fn main(){
    let _ = temp().await;
}