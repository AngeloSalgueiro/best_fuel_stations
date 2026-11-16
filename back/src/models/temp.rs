use reqwest::Error;

async fn fetch_data(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

pub async fn temp(){
    let url = "https://data.economie.gouv.fr/api/explore/v2.1/catalog/datasets/prix-des-carburants-en-france-flux-instantane-v2/records?limit=20";

    let result = fetch_data(url).await;
    
    let data = match result  {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {error:?}");
        }
    };

    println!("{}", data);
}
