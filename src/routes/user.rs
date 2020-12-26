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