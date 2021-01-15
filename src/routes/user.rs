use rocket_contrib::json::Json;
use serde_json::Value;
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
    Json(crate::achievements::achievements(&user.name))
}

#[get("/system_messages")]
pub fn system_messages(user: guards::User) -> Json<Value> {
    // TODO: Actual implementation

    use serde_json::Number;
    let mut message = json!({});
    message["id"] = Value::Number(Number::from(0));
    message["type"] = Value::String("text".to_string());
    message["timestamp"] = Value::Number(Number::from(crate::epoch()));
    message["content"] = Value::String("Hello World".to_string());

    use rand::Rng;
    let rand_num = rand::thread_rng().gen_range(0, 4);

    let mut result = Vec::new();

    match rand_num {
        0 => {}
        1 => {}
        2 => {
            result.push(message);
        }
        3 => {
            result.push(message.clone());
            message["id"] = Value::Number(Number::from(1));
            result.push(message); }
        _ => unreachable!()
    }

    Json(serde_json::to_value(result).unwrap())
}