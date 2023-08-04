#[derive(Debug)]
pub struct CommandLineOutput {
    pub amount: f64,
    pub foreign_currency: String,
    pub home_currency: String,
}

pub struct Data {
    pub config: Config,
}

pub struct Config {
    pub api_key: String,
}
