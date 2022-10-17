use teloxide::{prelude::*, utils::command::BotCommands};
use std::error::Error;
use cointoken_bot::get_price;

#[tokio::main]
async fn main() {
    env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::commands_repl(bot, answer, Command::ty()).await;
}

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "example: /price 1 btc to usd")]
    Help,
    #[command(description = "Get current price of a crypto currency.")]
    Price(String),
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => {
            bot.send_message(message.chat.id, Command::descriptions().to_string()).await?
        }
        Command::Price(cmd) => {
            log::debug!("cmd: {}", cmd);
            let mut amount = 1.0;
            let mut atoken = "BTC".to_string();
            let mut btoken = "USD".to_string();
            if cmd != "" {
                let cmd = cmd.split_whitespace().collect::<Vec<&str>>();
                amount = cmd[0].parse::<f64>()?;
                atoken = cmd[1].to_uppercase();
                btoken = cmd[3].to_uppercase();
            }
            let price = get_price(&atoken, &btoken, amount).await?;
            bot.send_message(
                message.chat.id,
                format!("{amount} ({atoken}) is {price} ({btoken})"),
            )
            .await?
        }
    };

    Ok(())
}