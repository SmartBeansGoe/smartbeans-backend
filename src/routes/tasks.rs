use rocket::http::Status;
use rocket_contrib::json::Json;
use serde_json::Value;
use crate::{guards, smartape};

#[get("/tasks?<solved>&<id>")]
pub fn get_tasks(user: guards::User, solved: Option<bool>, id: Option<i64>) -> Result<Json<Value>, Status> {
    let solved_tasks = smartape::progress(&user.token)?;
    let tasks = smartape::tasks(user.token)?;

    let tasks_filtered = tasks.into_iter()
        .map(|mut task| {
            let taskid = task["taskid"].as_i64().unwrap();
            let is_solved = solved_tasks.contains(&taskid);
            task["solved"] = serde_json::to_value(is_solved).unwrap();

            task
        })
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
pub fn submit(user: guards::User, taskid: String, data: rocket::Data) -> Result<String, Status> {
    let data = crate::data_to_string(data);
    smartape::submit(&user.token, &taskid, &data)?;

    crate::achievements::AchievementTrigger::new(&user)?.run("submission");

    Ok(String::new())
}

#[get("/submissions/<taskid>")]
pub fn submissions(user: guards::User, taskid: i64) -> Result<Json<Value>, Status> {
    let subs = smartape::all_submissions(&user.token)?.into_iter()
        .filter(|submission| {
            submission["taskid"].as_i64() == Some(taskid)
        })
        .collect::<Vec<_>>();

    Ok(Json(serde_json::to_value(subs).unwrap()))
}