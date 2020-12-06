use rocket_contrib::json::Json;
use serde_json::Value;
use rand::Rng;
use crate::guards;

/// Just an example
#[get("/username")]
pub fn get_username(user: guards::User) -> Json<Value> {
    Json(json!({
        "username": user.name
    }))
}

/// Another example
#[get("/rand/<min>/<max>")]
pub fn rand(min: i32, max: i32, _user: guards::User) -> Json<Value> {
    let rand_num = rand::thread_rng().gen_range(min, max);
    Json(json!({
        "rand": rand_num
    }))
}