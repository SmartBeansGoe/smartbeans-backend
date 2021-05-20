use telegram_bot::prelude::*;
use telegram_bot::{Api, Error, Message, MessageKind};
use reqwest::header::CONTENT_TYPE;
use diesel::prelude::*;
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

pub async fn new_error_message(api: Api, message: Message) -> Result<(), Error> {
    if let MessageKind::Text { ref data, .. } = message.kind {
        let split = data[10..].split("//").collect::<Vec<_>>();
        if split.len() != 2 {
            api.send(message.text_reply("Benutzung: /new_error <Titel> // <Nachricht>")).await?;
            return Ok(());
        }
        let new_title = split[0].trim();
        let new_content = split[1].trim();

        use crate::schema::error_messages::dsl::*;
        diesel::insert_into(error_messages)
            .values((
                title.eq(new_title),
                content.eq(new_content),
            ))
            .execute(&crate::database::establish_connection())
            .expect("Database error");

        api.send(message.text_reply("Erledigt")).await?;
        return Ok(());
    };

    api.send(message.text_reply("Benutzung: /new_error <Titel> // <Nachricht>")).await?;
    Ok(())
}

pub async fn list_error_messages(api: Api, message: Message) -> Result<(), Error> {
    use crate::schema::error_messages::dsl::*;
    let mut messages = error_messages.select((id, title, content))
        .load::<(i32, String, String)>(&crate::database::establish_connection())
        .expect("Database error")
        .into_iter()
        .fold(String::new(), |acc, message| {
            format!("{}{}: {}\n{}\n\n", acc, message.0, message.1, message.2)
        });

    if messages == "" {
        messages = "Keine Fehlermeldungen.".to_string();
    }

    api.send(message.text_reply(&messages)).await?;
    Ok(())
}

pub async fn delete_error_message(api: Api, message: Message) -> Result<(), Error> {
    if let MessageKind::Text { ref data, .. } = message.kind {
        let split = data[13..].split_whitespace().collect::<Vec<_>>();
        if split.len() != 1 {
            api.send(message.text_reply("Benutzung: /delete_error <id>")).await?;
            return Ok(());
        }
        if let Ok(message_id) = split[0].parse::<i32>() {
            use crate::schema::error_messages::dsl::*;

            if error_messages.filter(id.eq(message_id))
                .first::<(i32, String, String)>(&crate::database::establish_connection())
                .is_err() {
                api.send(message.text_reply("Ungültige id.")).await?;
                return Ok(());
            }

            diesel::delete(error_messages.filter(id.eq(message_id)))
                .execute(&crate::database::establish_connection())
                .expect("Database error");

            api.send(message.text_reply("Erledigt")).await?;
            return Ok(());
        }
    }

    api.send(message.text_reply("Benutzung: /delete_error <id>")).await?;
    Ok(())
}