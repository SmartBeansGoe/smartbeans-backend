use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::http::Status;
use serde_json::Value;
use crate::auth::guards;

pub mod tasks;
pub mod submissions;

#[get("/courses/<course>/meta")]
pub fn route_get_course_meta(course: String) -> Result<Json<Value>, Status> {
    use crate::schema::courses;
    let (title, config) = courses::table.filter(courses::name.eq(&course))
        .select((courses::title, courses::config))
        .first::<(String, String)>(&crate::database_connection())
        .or(Err(Status::NotFound))?;

    Ok(Json(json!({
        "name": course,
        "title": title,
        "config": (serde_json::from_str(&config) as serde_json::Result<Value>).unwrap()
    })))
}

#[get("/courses/<course>/progress")]
pub fn route_get_course_progress(user: guards::User, course: String) -> Result<Json<Vec<i32>>, Status> {
    if user.course != course {
        return Err(Status::Forbidden);
    }

    use crate::schema::submissions;
    let mut tasks = submissions::table.filter(submissions::course.eq(course))
        .filter(submissions::user.eq(user.name))
        .select(submissions::taskid)
        .load::<i32>(&crate::database_connection())
        .expect("Database error");

    tasks.sort();
    tasks.dedup();

    Ok(Json(tasks))
}

pub fn name_to_title(course: &str) -> Option<String> {
    use crate::schema::courses;
    courses::table.filter(courses::name.eq(&course))
        .select(courses::title)
        .first::<String>(&crate::database_connection())
        .ok()
}