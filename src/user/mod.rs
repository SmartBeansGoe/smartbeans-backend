use rocket::serde::json::Json;
use rocket::http::Status;
use serde_json::Value;
use diesel::prelude::*;
use crate::auth::guards;

pub mod character;

#[get("/user/meta")]
pub fn route_get_meta(user: guards::User) -> Result<Json<Value>, Status> {
    use crate::schema::users;
    let (display_name, password, lti_enabled) = users::table.filter(users::username.eq(&user.name))
        .select((users::displayName, users::password, users::ltiEnabled))
        .first::<(String, Option<String>, bool)>(&crate::database_connection())
        .expect("Database error");

    Ok(Json(json!({
        "username": user.name,
        "displayName": display_name,
        "passwordSet": password.is_some(),
        "ltiEnabled": lti_enabled,
        "activeCourse": {
            "name": user.course,
            "title": crate::course::name_to_title(&user.course)
        }
    })))
}

#[put("/user/displayName", data = "<data>")]
pub fn put_display_name(user: guards::User, data: Json<Value>) -> Result<Status, Status> {
    let display_name = data["displayName"].as_str()
        .ok_or(Status::BadRequest)?;

    use crate::schema::users;
    diesel::update(users::table.filter(users::username.eq(user.name)))
        .set(users::displayName.eq(display_name))
        .execute(&crate::database_connection())
        .expect("Database error");

    Ok(Status::Ok)
}