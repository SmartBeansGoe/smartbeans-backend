use rocket::http::Status;
use rocket_contrib::json::Json;
use serde_json::Value;
use diesel::prelude::*;
use crate::guards;
use crate::models::Character;

#[get("/character")]
pub fn get_character(user: guards::User) -> Json<Value> {
    let data = character_information(&user.name).expect("Character not initalized");

    let mut json = json!({});
    json["body_color"] = serde_json::to_value(data.body_color).unwrap();
    json["hat_id"] = serde_json::to_value(data.hat_id).unwrap();
    json["face_id"] = serde_json::to_value(data.face_id).unwrap();
    json["shirt_id"] = serde_json::to_value(data.shirt_id).unwrap();
    json["pants_id"] = serde_json::to_value(data.pants_id).unwrap();

    Json(json)
}

#[derive(Deserialize, Debug)]
pub struct CharacterJson {
    body_color: Option<String>,
    hat_id: Option<String>,
    face_id: Option<String>,
    shirt_id: Option<String>,
    pants_id: Option<String>
}

#[post("/character", data = "<data>")]
pub fn post_character(user: guards::User, data: Json<CharacterJson>) -> Status {
    use crate::schema::characters::dsl::*;
    let conn = diesel::sqlite::SqliteConnection::establish("db.sqlite").unwrap();

    // Because we rewrite every value anyway, we can just delete and recreate the dataset
    diesel::delete(characters.filter(username.eq(&user.name)))
        .execute(&conn)
        .expect("Database error");

    diesel::insert_into(characters)
        .values((
            username.eq(user.name),
            body_color.eq(&data.body_color),
            hat_id.eq(&data.hat_id),
            face_id.eq(&data.face_id),
            shirt_id.eq(&data.shirt_id),
            pants_id.eq(&data.pants_id)
            ))
        .execute(&conn)
        .expect("Database error");

    Status::Ok
}

pub fn character_information(user: &str) -> Option<Character> {
    use crate::schema::characters::dsl::*;
    let conn = diesel::sqlite::SqliteConnection::establish("db.sqlite").unwrap();

    characters.filter(username.eq(user))
        .first(&conn)
        .ok()
}

pub fn init_char(user: &str) {
    use crate::schema::characters::dsl::*;
    let conn = diesel::sqlite::SqliteConnection::establish("db.sqlite").unwrap();

    if character_information(user).is_none() {
        diesel::insert_into(characters)
            .values(username.eq(user))
            .execute(&conn)
            .expect("Database error");
    }
}