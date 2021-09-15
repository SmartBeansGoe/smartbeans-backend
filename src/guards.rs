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

        // Check if there is a valid session in the database
        use crate::schema::sessions::dsl::*;

        if let Ok(name) = sessions.filter(auth_token.eq(request_token))
            .filter(expiration_time.gt(crate::epoch()))
            .select(username)
            .first(&crate::database::establish_connection()) {
            Outcome::Success(User { name, token: request_token.to_string() })
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}