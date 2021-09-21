use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::http::Status;
use serde_json::Value;

pub mod tasks;

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

pub fn name_to_title(course: &str) -> Option<String> {
    use crate::schema::courses;
    courses::table.filter(courses::name.eq(&course))
        .select(courses::title)
        .first::<String>(&crate::database_connection())
        .ok()
}