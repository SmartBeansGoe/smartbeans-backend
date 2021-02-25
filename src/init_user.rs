use diesel::prelude::*;
use rand::Rng;
use crate::static_data::NAMES;

/// Initializes user. Use this only on first login, as this will panic if the user is already initialized.
/// (You can check this by looking up if there are already datasets for this user.)
pub fn init(user: &str, id: &str) {
    init_user(user, id);
    init_char(user);
    init_charname(user);
}

pub fn init_user(user: &str, id: &str) {
    use crate::schema::users::dsl::*;

    diesel::insert_into(users)
        .values((username.eq(user), studip_userid.eq(id)))
        .execute(&crate::database::establish_connection())
        .expect("Database error");
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
        .values((username.eq(user), charname.eq(NAMES[rand::thread_rng().gen_range(0, NAMES.len())])))
        .execute(&crate::database::establish_connection())
        .expect("Database error");
}