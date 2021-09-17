use super::guards;
use rocket::serde::json::Json;
use serde_json::Value;
use rocket::http::Status;
use rand::Rng;
use diesel::prelude::*;

#[post("/auth/register", data = "<data>")]
pub fn post_register(_key: guards::RegistrationKey, data: Json<Value>) -> Result<Status, Status> {
    let username = data["username"].as_str()
        .ok_or(Status::BadRequest)?;
    let password = data["password"].as_str()
        .ok_or(Status::BadRequest)?;
    let display_name = data["displayName"].as_str()
        .ok_or(Status::BadRequest)?;

    let password_hash = password_hash(password);

    if super::try_init_user(username, display_name, &Some(password_hash)) {
        Ok(Status::Ok)
    }
    else {
        Err(Status::Forbidden)
    }
}

#[post("/auth/login/password", data = "<data>")]
pub fn post_login_password(data: Json<Value>) -> Result <Json<Value>, Status> {
    let username = data["username"].as_str()
        .ok_or(Status::BadRequest)?;
    let password = data["password"].as_str()
        .ok_or(Status::BadRequest)?;
    let course = data["course"].as_str()
        .ok_or(Status::BadRequest)?;

    if !crate::course::exists(&course) {
        return Err(Status::NotFound);
    }

    use crate::schema::users;
    let hash = users::table.filter(users::username.eq(username))
        .select(users::password)
        .first::<Option<String>>(&crate::database_connection())
        .or(Err(Status::NotFound))?
        .ok_or(Status::Forbidden)?;

    if !password_verify(password, &hash) {
        return Err(Status::Unauthorized);
    }

    Ok(Json(json!({
        "token": super::create_session(username, course, &None)
    })))
}

#[put("/auth/password", data = "<data>")]
pub fn put_password(user: guards::User, data: Json<Value>) -> Result<Status, Status> {
    let new_password = data["newPassword"].as_str()
        .ok_or(Status::BadRequest)?;

    use crate::schema::users;
    diesel::update(users::table.filter(users::username.eq(user.name)))
        .set(users::password.eq(password_hash(new_password)))
        .execute(&crate::database_connection())
        .expect("Database error");

    Ok(Status::Ok)
}

fn password_hash(password: &str) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let password = password.as_bytes();
    let config = argon2::Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

fn password_verify(password: &str, hash: &str) -> bool {
    argon2::verify_encoded(hash, password.as_bytes()).unwrap()
}