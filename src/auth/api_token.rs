use rocket::serde::json::Json;
use super::guards;
use diesel::prelude::*;
use diesel::dsl::not;
use serde_json::Value;
use rocket::http::Status;

#[post("/auth/apiToken/<token_name>")]
pub fn post_api_token(user: guards::User, token_name: String) -> Json<Value> {
    Json(json!({
        "apiToken": super::create_session(&user.name, &user.course, &Some(token_name))
    }))
}

#[get("/auth/apiToken")]
pub fn get_api_token(user: guards::User) -> Json<Value> {
    use crate::schema::sessions;
    let token_names = sessions::table.filter(sessions::username.eq(&user.name))
        .filter(not(sessions::tokenName.is_null()))
        .select(sessions::tokenName)
        .load::<Option<String>>(&crate::database_connection())
        .expect("Database error");

    Json(serde_json::to_value(token_names).unwrap())
}

#[delete("/auth/apiToken/<token_name>")]
pub fn delete_api_token(user: guards::User, token_name: String) -> Status {
    use crate::schema::sessions;
    diesel::delete(sessions::table.filter(sessions::tokenName.eq(&token_name)))
        .filter(sessions::username.eq(user.name))
        .execute(&crate::database_connection())
        .expect("Database error");

    Status::Ok
}