use percent_encoding::{utf8_percent_encode as perc_encode, AsciiSet, NON_ALPHANUMERIC};
use hmac::{Hmac, Mac, NewMac};
use crypto_hashes::sha2::Sha256;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use serde_json::Value;
use rocket::http::Status;
use diesel::prelude::*;
use std::collections::BTreeMap;
use crate::tools::{epoch, data_to_string};
use crate::SETTINGS;
use crate::auth::guards;

#[post("/auth/login/lti", data = "<data>")]
pub async fn auth_lti(data: rocket::Data<'_>) -> Result<Redirect, Status> {
    let data = data_to_string(data).await;
    let lti_params: BTreeMap<String, String> = serde_urlencoded::from_str(&data).unwrap();
    let lti_url: String = SETTINGS.get("auth.lti.url")
        .expect("auth.lti.url not found in settings");
    let lti_secret: String = SETTINGS.get("auth.lti.secret")
        .expect("auth.lti.secret not found in settings");

    if !validate_lti(&lti_url, lti_params.clone(), &lti_secret) {
        return Err(Status::Unauthorized);
    }

    let username = &lti_params["lis_person_sourcedid"];

    super::try_init_user(
        username,
        &lti_params["lis_person_name_given"],
        &None
    );

    use crate::schema::users;
    let lti_enabled = users::table.filter(users::username.eq(username))
        .select(users::ltiEnabled)
        .first::<bool>(&crate::database_connection())
        .expect("Database error");

    if !lti_enabled {
        return Err(Status::Forbidden);
    }

    use crate::schema::courseMapping;
    let course = courseMapping::table.filter(courseMapping::studipId.eq(&lti_params["context_id"]))
        .select(courseMapping::courseName)
        .first::<String>(&crate::database_connection())
        .expect("Database error: Probably missing course mapping");

    let token = super::create_session(username, &course, &None);
    let redirect_url = SETTINGS.get::<String>("auth.lti.redirect")
        .expect("auth.lti.redirect not found in settings");

    Ok(Redirect::to(format!("{}#{}", redirect_url, token)))
}

#[put("/auth/ltiEnabled", data = "<data>")]
pub fn put_lti_status(user: guards::User, data: Json<Value>) -> Result<Status, Status> {
    let new_status = data["ltiEnabled"].as_bool()
        .ok_or(Status::BadRequest)?;

    use crate::schema::users;
    diesel::update(users::table.filter(users::username.eq(user.name)))
        .set(users::ltiEnabled.eq(new_status))
        .execute(&crate::database_connection())
        .expect("Database error");

    Ok(Status::Ok)
}

fn validate_lti(uri: &str, mut params: BTreeMap<String, String>, secret: &str) -> bool {
    let studip_signature = params.remove("oauth_signature").unwrap();
    let timestamp = params["oauth_timestamp"].parse::<i64>().unwrap();

    // We want to percent encode all characters except 'A-Z', 'a-z',
    // '0-9', '-', ',', '_', '~' (see RFC 3986)
    const FRAGMENT: &AsciiSet = &NON_ALPHANUMERIC
        .remove(b'-')
        .remove(b'.')
        .remove(b'_')
        .remove(b'~');

    let params_encoded = params.iter()
        .map(|(k, v)| format!("{}={}", k, perc_encode(&v, FRAGMENT)))
        .collect::<Vec<_>>()
        .join("&");

    let base_string = format!("POST&{}&{}",
                              perc_encode(uri, FRAGMENT),
                              perc_encode(&params_encoded, FRAGMENT)
    );

    // Calculate the signature of our request.
    let secret = format!("{}&", perc_encode(secret, FRAGMENT));
    let mut mac = Hmac::<Sha256>::new_varkey(secret.as_bytes()).unwrap();
    mac.update(base_string.as_bytes());
    let request_signature = base64::encode(mac.finalize().into_bytes());

    request_signature == studip_signature && timestamp + 1800 > epoch()
}