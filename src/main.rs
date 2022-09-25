use bpaf::Bpaf;
use std::error::Error;
use serde_json::Value;

#[derive(Clone, Debug, Bpaf)]
#[bpaf(options, version)]
/// Convert the price of any two tokens
struct FromTokenAndToToken {
    // From Token
    from: String,
    // To Token
    to: String,
    // Amount
    amount: f64,
}


fn get_price(
    from_token: &str,
    to_token: &str,
    amount: f64,
) -> Result<f64, Box<dyn Error>> {
    let url = format!(
        "https://pro-api.coinmarketcap.com/v1/tools/price-conversion?amount={}&symbol={}&convert={}",
        amount, from_token, to_token
    );
    let client = reqwest::blocking::Client::new();
    let cmc_apikey = std::env::var("CMC_APIKEY")?;
    let res = client
        .get(&url)
        .header("X-CMC_PRO_API_KEY", cmc_apikey)
        .send()?;
    let body = res.text()?;
    let json: Value = serde_json::from_str(&body)?;
    let price = json["data"]["quote"][to_token]["price"].as_f64().unwrap();
    Ok(price)
}

fn main() {
    let opts = from_token_and_to_token().run();
    let price = get_price(&opts.from, &opts.to, opts.amount).unwrap();
    println!("{} {} == {} {}", opts.amount, opts.from, price, opts.to);
}
