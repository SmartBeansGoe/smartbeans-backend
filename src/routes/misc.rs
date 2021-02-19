use rocket_contrib::json::Json;
use serde_json::Value;
use rocket::http::Status;
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

/// Returns an error
#[get("/error/<code>")]
pub fn error(code: u16) -> Status {
    Status::from_code(code).unwrap_or(Status::NotFound)
}