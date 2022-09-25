use bpaf::Bpaf;
use cointoken_bot::get_price;

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

fn main() {
    let opts = from_token_and_to_token().run();
    let price = get_price(&opts.from, &opts.to, opts.amount).unwrap();
    println!("{} {} == {} {}", opts.amount, opts.from, price, opts.to);
}