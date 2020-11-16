use rocket_contrib::json::Json;
use serde_json::Value;
use rand::Rng;
use crate::guards;

/// Just an example
#[get("/username")]
pub fn get_username(username: guards::Username) -> Json<Value> {
    Json(json!({
        "username": username.0
    }))
}

/// Another example
#[get("/rand/<min>/<max>")]
pub fn rand(min: i32, max: i32, _username: guards::Username) -> Json<Value> {
    let rand_num = rand::thread_rng().gen_range(min, max);
    Json(json!({
        "rand": rand_num
    }))
}