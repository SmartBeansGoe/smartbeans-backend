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
        message_json["timestamp"] = Value::Number(Number::from(message.time));
        message_json["content"] = match &message.message_type[..] {
            "achievement_unlocked" => serde_json::from_str(&message.content).unwrap(),
            _ => Value::String(message.content)
        };
        message_json["type"] = Value::String(message.message_type);

        messages_json.push(message_json);
    }

    Json(serde_json::to_value(messages_json).unwrap())
}

#[get("/message_test")]
pub fn message_test(user: guards::User) -> Json<Value> {
    let mut achievement = json!({});
    achievement["id"] = Value::Number(Number::from(42));
    achievement["name"] = Value::String("Test".to_string());
    achievement["description"] = Value::String("Testen ist wichtig!".to_string());
    achievement["completed"] = Value::Number(Number::from(234567));

    crate::system_messages::send_message(
        &user.name,
        "achievement_unlocked",
        &achievement.to_string()
    );

    system_messages(user)
}