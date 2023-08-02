// get data from API
async fn get_coversion_data() -> Result<(), reqwest::Error> {
    let conversion = reqwest::Client::new()
        .get("https://api.freecurrencyapi.com/v1/latest?apikey=fca_live_hMe1XrPWnPH2QQSXRUh3E8QIvD8tjlYa4h8PdnPE")
        .send()
        .await?
        .text()
        .await?;
    println!("{:#?", conversion);

    Ok(())
}
