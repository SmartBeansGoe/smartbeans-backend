use std::time::{SystemTime, UNIX_EPOCH};

use rocket::request::{Request, FromRequest, Outcome};
use rocket::http::Status;
use diesel::prelude::*;

/// Request guard for authentication
#[derive(Debug)]
pub struct User {
    pub name: String,
    pub token: String
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    /// Returns a guards::User when the user is logged in,
    /// 400 on invalid Authorization header (missing or wrong Syntax)
    /// 401 on invalid auth token
    fn from_request(request: &'a Request<'r>) -> Outcome<User, Self::Error> {
        // Get auth token from Authorization header
        let auth_header = request.headers()
            .get("Authorization")
            .next();

        if auth_header.is_none() {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let auth_header = auth_header.unwrap()
            .split_whitespace()
            .collect::<Vec<_>>();

        if auth_header.len() != 2 || auth_header[0] != "Bearer" {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let request_token = auth_header[1];

        // Get auth token from cookie
        // (alternative to Authorization header; just in case we need this in the future)
        /*let cookies = Cookies::from_request(request).unwrap();
        let cookie_token = cookies.get("auth_token");

        if cookie_token.is_none() {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        let request_token = cookie_token.unwrap().value();*/

        // Check if there is a valid session in the database
        let epoch_time_now = SystemTime::now().duration_since(UNIX_EPOCH)
            .unwrap().as_secs() as i64;

        use crate::schema::sessions::dsl::*;
        let conn = crate::MainDbConn::from_request(request).unwrap();
        if let Ok(name) = sessions.filter(auth_token.eq(request_token))
            .filter(expiration_time.gt(epoch_time_now))
            .select(username)
            .first(&*conn) {
            Outcome::Success(User { name, token: request_token.to_string() })
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}