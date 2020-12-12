use rocket::http::Status;
use rocket_contrib::json::Json;
use serde_json::Value;
use crate::{guards, smartape};

#[get("/tasks?<solved>&<id>")]
pub fn get_tasks(user: guards::User, solved: Option<bool>, id: Option<i64>) -> Result<Json<Value>, Status> {
    let tasks = smartape::tasks(&user.token)?;
    let tasks_filtered = tasks.as_array().unwrap()
        .iter()
        .filter(|task| solved.is_none() || task["solved"].as_bool() == Some(solved.unwrap()))
        .filter(|task| id.is_none() || task["taskid"].as_i64() == Some(id.unwrap()))
        .collect::<Vec<_>>();

    Ok(Json(serde_json::to_value(tasks_filtered).unwrap()))
}

#[get("/progress")]
pub fn progress(user: guards::User) -> Result<String, Status> {
    Ok(format!("{:?}", smartape::progress(&user.token)?))
}

#[post("/submit/<taskid>", format = "text/plain", data = "<data>")]
pub fn submit(user:guards::User, taskid: String, data: String) -> Result<String, Status> {
    smartape::submit(&user.token, &taskid, &data)?;

    Ok(String::new())
}