use telegram_bot::prelude::*;
use telegram_bot::{Api, Error, Message, MessageKind, ParseMode, UpdateKind};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = "1600088858:AAGf2GCvTJb4KKx59cBDq1DrdY1ZrxeeT0c";

    let api = Api::new(token);
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
            "/version" => version_message(api, message).await?,
            _ => (),
        },
        _ => (),
    };

    Ok(())
}

async fn version_message(api: Api, message: Message) -> Result<(), Error> {
    api.send(message.text_reply(env!("GIT_HASH"))).await?;

    Ok(())
}