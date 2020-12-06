use rocket::http::Status;
use rocket_contrib::json::Json;
use serde_json::Value;
use crate::{guards, smartape};

#[get("/tasks")]
pub fn get_tasks(user: guards::User) -> Result<Json<Value>, Status> {
    Ok(Json(smartape::tasks(&user.token)?))
}

#[get("/task/<id>")]
pub fn get_task(user: guards::User, id: String) -> Result<Json<Value>, Status> {
    Ok(Json(smartape::task(&user.token, &id)?))
}