use rocket_contrib::json::Json;
use serde_json::{Value, Number};
use crate::guards;

/// Returns the username
#[get("/username")]
pub fn get_username(user: guards::User) -> Json<Value> {
    Json(json!({
        "username": user.name
    }))
}

/// Return all achievements for the user
#[get("/achievements")]
pub fn get_achievements(user: guards::User) -> Json<Value> {
    Json(serde_json::to_value(crate::achievements::achievements(&user.name)).unwrap())
}

#[get("/system_messages")]
pub fn system_messages(user: guards::User) -> Json<Value> {
    let messages = crate::system_messages::receive_messages(&user.name);
    let mut messages_json = Vec::new();

    for message in messages {
        let mut message_json = json!({});
        message_json["id"] = Value::Number(Number::from(message.id));
        message_json["type"] = Value::String(message.message_type);
        message_json["timestamp"] = Value::Number(Number::from(message.time));
        message_json["content"] = Value::String(message.content);

        messages_json.push(message_json);
    }

    Json(serde_json::to_value(messages_json).unwrap())
}