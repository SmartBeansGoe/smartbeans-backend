use telegram_bot::{Api, Error, Message, MessageKind, UpdateKind};
use futures::StreamExt;
use std::env;
use smartbeans_backend::bot;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Load environment variables from .env
    dotenv::dotenv().ok();

    // Load default values from .env-default
    if dotenv::from_filename(".env-default").is_err() {
        println!("Error: .env-default not found. Exiting...");
        std::process::exit(1);
    }

    let token = env::var("BOT_TOKEN").expect("Environment variable BOT_TOKEN not found!");

    let api = Api::new(&token);
    let mut stream = api.stream();

    while let Some(update) = stream.next().await {
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            read_message(api.clone(), message).await?;
        }
    }

    Ok(())
}

async fn read_message(api: Api, message: Message) -> Result<(), Error> {
    if let MessageKind::Text { ref data, .. } = message.kind {
        match data.split_whitespace().next() {
            Some("/version") => bot::version_message(api, message).await?,
            Some("/chatid") => bot::chatid_message(api, message).await?,
            Some("/new_error") => bot::new_error_message(api, message).await?,
            Some("/list_errors") => bot::list_error_messages(api, message).await?,
            Some("/delete_error") => bot::delete_error_message(api, message).await?,
            _ => (),
        }
    };

    Ok(())
}