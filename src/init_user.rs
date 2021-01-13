use diesel::prelude::*;

/// Initializes user. Use this only on first login, as this will panic if the user is already initialized.
/// (You can check this by looking up if there are already datasets for this user.)
pub fn init(user: &str) {
    init_char(user);
    init_charname(user);
}

/// Initializes character with null in every field. Panics if there is already a char dataset for this user.
pub fn init_char(user: &str) {
    use crate::schema::characters::dsl::*;

    diesel::insert_into(characters)
        .values(username.eq(user))
        .execute(&crate::database::establish_connection())
        .expect("Database error");
}

/// Initializes charname with username. Panics if charname for this user is already set.
pub fn init_charname(user: &str) {
    use crate::schema::charnames::dsl::*;

    diesel::insert_into(charnames)
        .values((username.eq(user), charname.eq(user)))
        .execute(&crate::database::establish_connection())
        .expect("Database error");
}

// === Debug stuff. We will probably remove this sooner or later. (TODO)

/// Initializes user. If the user is already initalized, the data will be overwritten.
pub fn reinit(user: &str) {
    use crate::schema::{characters, charnames};
    let conn = crate::database::establish_connection();

    diesel::delete(charnames::dsl::charnames.filter(charnames::dsl::username.eq(user)))
        .execute(&conn).ok();

    diesel::delete(characters::dsl::characters.filter(characters::dsl::username.eq(user)))
        .execute(&conn).ok();

    init(user);
}

#[get("/reinit")]
pub fn reinit_route(user: crate::guards::User) -> rocket::http::Status {
    reinit(&user.name);

    rocket::http::Status::Ok
}