use rocket::request::{Request, FromRequest, Outcome};
use rocket::http::Status;
use diesel::prelude::*;
use crate::SETTINGS;

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub course: String
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = get_token(req);
        let token = if let Err(status) = token {
            return Outcome::Failure((status, ()));
        }
        else {
            token.unwrap()
        };

        if !super::check_and_refresh_token(&token) {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        use crate::schema::sessions;
        let (username, course_name) = sessions::table.filter(sessions::token.eq(token))
            .select((sessions::username, sessions::courseName))
            .first::<(String, String)>(&crate::database_connection())
            .expect("Database error");

        Outcome::Success(User {
            name: username,
            course: course_name
        })
    }
}

#[derive(Debug)]
pub struct AdminKey { }

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminKey {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = get_token(req);
        let token = if let Err(status) = token {
            return Outcome::Failure((status, ()));
        }
        else {
            token.unwrap()
        };

        let admin_key = SETTINGS.get::<String>("auth.admin_key")
            .expect("auth.admin_key not found in settings");

        if admin_key == "" {
            return Outcome::Failure((Status::Forbidden, ()));
        }

        if admin_key == token {
            return Outcome::Success(AdminKey { });
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}

#[derive(Debug)]
pub struct RegistrationKey { }

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RegistrationKey {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if !SETTINGS.get::<bool>("auth.password.key_required").unwrap() {
            return Outcome::Success(RegistrationKey { });
        }

        let token = get_token(req);
        let token = if let Err(status) = token {
            return Outcome::Failure((status, ()));
        }
        else {
            token.unwrap()
        };

        let keys = SETTINGS.get::<Vec<String>>("auth.password.registration_keys")
            .expect("auth.debug_key not found in settings");

        if keys.contains(&token) {
            return Outcome::Success(RegistrationKey { });
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}

fn get_token(req: &Request<'_>) -> Result<String, Status> {
    // Get auth token from Authorization header
    let auth_header = req.headers()
        .get("Authorization")
        .next();

    if auth_header.is_none() {
        return Err(Status::BadRequest);
    }

    let auth_header = auth_header.unwrap()
        .split_whitespace()
        .collect::<Vec<_>>();

    if auth_header.len() != 2 || auth_header[0] != "Bearer" {
        return Err(Status::BadRequest);
    }

    Ok(auth_header[1].to_string())
}