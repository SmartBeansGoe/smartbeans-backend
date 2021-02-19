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
    match message.kind {
        MessageKind::Text { ref data, .. } => match data.as_str() {
            "/version" => bot::version_message(api, message).await?,
            "/chatid" => bot::chatid_message(api, message).await?,
            _ => (),
        },
        _ => (),
    };

    Ok(())
}