use rocket_contrib::json::Json;
use serde_json::Value;
use rand::Rng;
use crate::guards;

/// Another example
#[get("/rand/<min>/<max>")]
pub fn rand(min: i32, max: i32, _user: guards::User) -> Json<Value> {
    let rand_num = rand::thread_rng().gen_range(min, max);
    Json(json!({
        "rand": rand_num
    }))
}