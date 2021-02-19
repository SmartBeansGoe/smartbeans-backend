use rocket_contrib::json::Json;
use serde_json::Value;
use rocket::http::Status;
use rand::Rng;
use crate::guards;
use diesel::prelude::*;

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

#[post("/report_error", format = "text/plain", data = "<data>")]
pub fn report_error(user: guards::User, data: rocket::Data) -> Status {
    use crate::schema::error_reports::dsl::*;

    let data = crate::data_to_string(data);

    diesel::insert_into(error_reports)
        .values((
            username.eq(user.name),
            time.eq(crate::epoch()),
            message.eq(&data)
        ))
        .execute(&crate::database::establish_connection())
        .expect("Database error");

    crate::bot::report_error(&data);

    Status::Ok
}