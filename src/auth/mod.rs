use rand::{thread_rng, Rng, distributions::Alphanumeric};
use diesel::prelude::*;
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use crate::SETTINGS;
use rocket::http::Status;

pub mod password;
pub mod lti;
pub mod guards;
pub mod api_token;

#[post("/auth/login/debug/<username>/<course>")]
pub fn auth_debug(username: String, course: String, _key: guards::DebugKey) -> Result<String, Status> {
    use crate::schema::users;
    users::table.filter(users::username.eq(&username))
        .select(users::username)
        .first::<String>(&crate::database_connection())
        .or(Err(Status::NotFound))?;

    use crate::schema::courses;
    courses::table.filter(courses::name.eq(&course))
        .select(courses::name)
        .first::<String>(&crate::database_connection())
        .or(Err(Status::NotFound))?;

    Ok(create_session(&username, &course, &None))
}

#[delete("/auth/logout/<token>")]
pub fn logout(token: String) -> Status {
    use crate::schema::sessions;
    diesel::delete(sessions::table.filter(sessions::token.eq(&token)))
        .execute(&crate::database_connection())
        .expect("Database error");

    Status::Ok
}

fn create_session(user: &str, course: &str, token_name: &Option<String>) -> String {
    let token: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    use crate::schema::sessions;
    diesel::insert_into(sessions::table)
        .values((
            sessions::token.eq(&token),
            sessions::username.eq(user),
            sessions::courseName.eq(course),
            sessions::expirationTime.eq(expiration_time() as i64),
            sessions::tokenName.eq(token_name)
        ))
        .execute(&crate::database_connection())
        .expect("Database error");

    token
}

fn check_and_refresh_token(token: &str) -> bool {
    use crate::schema::sessions;
    let result = sessions::table.filter(sessions::token.eq(token))
        .select((sessions::expirationTime, sessions::tokenName))
        .first::<(i64, Option<String>)>(&crate::database_connection());

    if result.is_err() {
        return false;
    }

    let (current_expiration_time, token_name) = result.unwrap();

    // Return false if the expiration time is in the past and the token is not a permanent API token
    if token_name.is_none() && current_expiration_time < crate::tools::epoch() {
        return false;
    }

    // Otherwise refresh the expiration time of the token
    diesel::update(sessions::table.filter(sessions::token.eq(token)))
        .set(sessions::expirationTime.eq(expiration_time() as i64))
        .execute(&crate::database_connection())
        .expect("Database error");

    true
}

fn expiration_time() -> u64 {
    let duration: u64 = SETTINGS.get("auth.session_duration")
        .expect("auth.session_duration not found in settings");
    (SystemTime::now() + Duration::new(duration, 0))
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Initializes a new user. Returns false if the username already exists.
/// Use password = None if the user logged in via LTI.
fn try_init_user(username: &str, display_name: &str, password_hash: &Option<String>) -> bool {
    let char_data = json!({
        "bodyColor": null,
        "hatId": null,
        "faceId": null,
        "shirtId": null,
        "pantsId": null
    });

    use crate::schema::users;
    let user_exists = users::table.filter(users::username.eq(username))
        .select(users::username)
        .first::<String>(&crate::database_connection())
        .is_ok();

    if user_exists {
        return false;
    }

    diesel::insert_into(users::table)
        .values((
            users::username.eq(username),
            users::displayName.eq(display_name),
            users::password.eq(password_hash),
            users::ltiEnabled.eq(password_hash.is_none()),
            users::charData.eq(char_data.to_string())
        ))
        .execute(&crate::database_connection())
        .expect("Database error");

    true
}