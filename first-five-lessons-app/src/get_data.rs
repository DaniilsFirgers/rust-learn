use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrenciesApiResponse {
    pub data: HashMap<String, f64>,
}

// get data from API
pub async fn get_coversion_data() -> Result<(), reqwest::Error> {
    let conversion = reqwest::Client::new()
        .get("https://api.freecurrencyapi.com/v1/latest?apikey=fca_live_hMe1XrPWnPH2QQSXRUh3E8QIvD8tjlYa4h8PdnPE")
        .send()
        .await?
        .json::<CurrenciesApiResponse>()
        .await?;
    println!("{:#?}", conversion);

    Ok(())
}
