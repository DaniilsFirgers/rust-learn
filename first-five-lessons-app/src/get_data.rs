use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrenciesApiResponse {
    pub data: HashMap<String, f64>,
}

// get data from API
#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub config: Config,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
}

pub async fn get_coversion_data(config: Data) -> Result<(), reqwest::Error> {
    let api_key = config.config.api_key;
    let client = reqwest::Client::new();

    let mut params = HashMap::new();
    params.insert("apikey", api_key);

    let conversion = client
        .get("https://api.freecurrencyapi.com/v1/latest")
        .query(&params)
        .send()
        .await?
        .json::<CurrenciesApiResponse>()
        .await?;
    println!("{:#?}", conversion);

    Ok(())
}

pub fn parse_config() -> Data {
    let config: Data = {
        let config_test =
            fs::read_to_string("config.toml").expect("Error while parsing config file.");
        toml::from_str(&config_test).expect("Error")
    };
    config
}
