use telegram_bot::prelude::*;
use telegram_bot::{Api, Error, Message};
use reqwest::header::CONTENT_TYPE;
use std::env;

pub async fn version_message(api: Api, message: Message) -> Result<(), Error> {
    api.send(message.text_reply(env!("GIT_HASH"))).await?;

    Ok(())
}

pub async fn chatid_message(api: Api, message: Message) -> Result<(), Error> {
    api.send(message.text_reply(message.chat.id().to_string())).await?;

    Ok(())
}

pub fn report_error(message: &str) {
    let message = format!("Neue Fehlermeldung:\n\n{}", message);
    // I don't understand how to send messages with telegram-bot, so I decided to just call the API directly.
    reqwest::blocking::Client::new()
        .post(&format!("https://api.telegram.org/bot{}/sendMessage", env::var("BOT_TOKEN").unwrap()))
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(format!("chat_id={}&disable_web_page_preview=1&text={}", env::var("TELEGRAM_CHAT").unwrap(), message))
        .send().ok();
}