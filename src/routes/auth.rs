use std::time::{SystemTime, Duration, UNIX_EPOCH};

use rocket::request::LenientForm;
use rocket::response::Redirect;
use rocket::http::{Status, Cookie, Cookies};
use rand::{thread_rng, Rng, distributions::Alphanumeric};
use diesel::prelude::*;
use rocket_contrib::json::Json;
use serde_json::Value;
use std::env;

use crate::models;

#[cfg(debug_assertions)]
#[get("/auth_debug/<username>")]
pub fn auth_debug(username: String, conn: crate::MainDbConn) -> Json<Value> {
    Json(json!({
        "auth_token": create_session(username, conn)
    }))
}

#[cfg(not(debug_assertions))]
#[get("/auth_debug/<_username>")]
pub fn auth_debug(_username: String) -> Status {
    Status::NotFound
}

#[get("/auth_debug/<username>/<key>")]
pub fn auth_debug_production(username: String, key: String,conn: crate::MainDbConn) -> Result<Json<Value>, Status> {
    if key != env::var("DEBUG_ACCESS_KEY").unwrap() || key == "changeme" {
        return Err(Status::Unauthorized);
    }

    Ok(Json(json!({
        "auth_token": create_session(username, conn)
    })))
}

#[derive(FromForm)]
pub struct LTIData {
    lis_person_sourcedid: String,
    oauth_signature: String
}

/// Takes an LTI post request and checks if the containing data is valid. If the authentication is
/// successful, a cookie with an auth token is set and the user gets redirected to the frontend.
/// Otherwise a 401 error is returned.
#[post("/auth_cookie", data = "<data>")]
pub fn auth_cookie(mut cookies: Cookies, data: LenientForm<LTIData>, conn: crate::MainDbConn) -> Result<Redirect, Status> {
    if !validate_lti() {
        return Err(Status::Unauthorized);
    }

    cookies.add(Cookie::new("auth_token", create_session(data.lis_person_sourcedid.clone(), conn)));
    Ok(Redirect::to(env::var("FRONTEND_URL").unwrap()))
}

/// Similar to auth_cookie, but instead of a redirect to the frontend, the auth token is returned.
#[post("/auth_token", data = "<data>")]
pub fn auth_token(data: LenientForm<LTIData>, conn: crate::MainDbConn) -> Result<Json<Value>, Status> {
    if !validate_lti() {
        return Err(Status::Unauthorized);
    }

    Ok(Json(json!({
        "auth_token": create_session(data.lis_person_sourcedid.clone(), conn)
    })))
}

fn validate_lti() -> bool {
    // TODO
    true
}

// Creates a session for a user and returns the auth token
fn create_session(username: String, conn: crate::MainDbConn) -> String {
    // Create random auth token
    let auth_token: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .collect();

    // expiration_time = now + 8h in epoch time
    let duration = env::var("TOKEN_DURATION").unwrap().parse::<u64>().expect("Invalid TOKEN_DURATION in .env");
    let expiration_time = (SystemTime::now() + Duration::new(60 * duration, 0))
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Write into session table
    diesel::insert_into(crate::schema::sessions::table)
        .values(&models::NewSession {
            auth_token: auth_token.clone(),
            expiration_time: expiration_time as i64,
            username,
        })
        .execute(&*conn)
        .unwrap();

    auth_token
}