use std::time::{SystemTime, Duration, UNIX_EPOCH};
use std::env;

use rocket::response::Redirect;
use rocket::http::{Status, Cookie, Cookies};
use rand::{thread_rng, Rng, distributions::Alphanumeric};
use diesel::prelude::*;
use rocket_contrib::json::Json;
use serde_json::Value;
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use hmac::{Hmac, Mac, NewMac};
use crypto_hashes::sha1::Sha1;

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

/// Takes an LTI post request and checks if the containing data is valid. If the authentication is
/// successful, a cookie with an auth token is set and the user gets redirected to the frontend.
/// Otherwise a 401 error is returned.
#[post("/auth_cookie", data = "<data>")]
pub fn auth_cookie(mut cookies: Cookies, data: String, conn: crate::MainDbConn) -> Result<Redirect, Status> {
    let username = validate_lti(
        &format!("{}/auth_cookie", env::var("BACKEND_URL").unwrap()),
        &data,
        &env::var("LTI_SECRET").unwrap()
    )?;

    let auth_token = create_session(username, conn);
    cookies.add(
        Cookie::build("auth_token", auth_token)
            .path("/")
            .finish()
    );

    Ok(Redirect::to(env::var("FRONTEND_URL").unwrap()))
}

/// Similar to auth_cookie, but instead of a redirect to the frontend, the auth token is returned.
#[post("/auth_token", data = "<data>")]
pub fn auth_token(data: String, conn: crate::MainDbConn) -> Result<Json<Value>, Status> {
    let username = validate_lti(
        &format!("{}/auth_token", env::var("BACKEND_URL").unwrap()),
        &data,
        &env::var("LTI_SECRET").unwrap()
    )?;

    Ok(Json(json!({
        "auth_token": create_session(username, conn)
    })))
}

/// Validates an LTI post request. Returns the username if the validation was successful, 401 if the
/// validation failed, 400 if the `lis_person_sourcedid` field is missing.
/// Warning: This is custom-tailored for Stud.IP and might not work for other systems.
///
/// * `uri`: The URI the request was sent to.
/// * `params`: The body of the POST request
/// * `secret`: The LTI consumer secret
///
/// TODO: Use timestamp and nonce to prevent reuse of the same request.
fn validate_lti(uri: &str, params: &str, secret: &str) -> Result<String, Status> {
    let mut params = serde_urlencoded::from_str::<Vec<(String, String)>>(
        // We need to do this, because Stud.IP calculates the signature using "\n" for line breaks,
        // but sends "\r\n".
        &params.replace("%0D%0A", "%0A")
    ).unwrap();

    // Get the oauth signature Stud,IP calculated
    let signature_index = params.iter()
        .position(|e| e.0 == "oauth_signature")
        .ok_or(Status::Unauthorized)?;
    let studip_signature = params.remove(signature_index).1;

    params.sort_unstable();

    let params_encoded = params.iter()
        .map(|(k, v)| format!("{}={}", k, perc_encode(&v)))
        .collect::<Vec<_>>()
        .join("&");

    let base_string = format!("POST&{}&{}", perc_encode(uri), perc_encode(&params_encoded));

    // Calculate the signature of our request.
    let secret = format!("{}&", perc_encode(secret));
    let mut mac = Hmac::<Sha1>::new_varkey(secret.as_bytes()).unwrap();
    mac.update(base_string.as_bytes());
    let request_signature = base64::encode(mac.finalize().into_bytes());

    if request_signature != studip_signature {
        return Err(Status::Unauthorized);
    }

    Ok(
        params.iter()
        .find(|e| e.0 == "lis_person_sourcedid")
        .ok_or(Status::BadRequest)?
        .clone().1
    )
}

/// Percent encodes a &str.
fn perc_encode(input: &str) -> String {
    // We want to percent encode all characters except 'A-Z', 'a-z', '0-9', '-', ',', '_', '~'
    // (see RFC 3986)
    const FRAGMENT: &AsciiSet = &NON_ALPHANUMERIC
        .remove(b'-')
        .remove(b'.')
        .remove(b'_')
        .remove(b'~');

    utf8_percent_encode(input, FRAGMENT).to_string()
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