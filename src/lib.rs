use std::error::Error;
use serde_json::Value;

pub async fn get_price(
    from_token: &str,
    to_token: &str,
    amount: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let url = format!(
        "https://pro-api.coinmarketcap.com/v1/tools/price-conversion?amount={}&symbol={}&convert={}",
        amount, from_token, to_token
    );
    let client = reqwest::Client::new();
    let cmc_apikey = std::env::var("CMC_APIKEY")?;
    let res = client
        .get(&url)
        .header("X-CMC_PRO_API_KEY", cmc_apikey)
        .send()
        .await?;
    let body = res.text().await?;
    let json: Value = serde_json::from_str(&body)?;
    let price = json["data"]["quote"][to_token]["price"].as_f64().unwrap();
    Ok(price)
}