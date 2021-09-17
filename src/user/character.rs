use rocket::serde::json::Json;
use rocket::http::Status;
use diesel::prelude::*;
use serde::{Serialize, Deserialize, Deserializer};
use crate::auth::guards;

#[get("/user/character")]
pub fn route_get_character(user: guards::User) -> Json<Character> {
    Json(get_character_data(&user.name))
}

#[patch("/user/character", data = "<patch>")]
pub fn route_patch_character(user: guards::User, patch: Json<CharacterPatch>) -> Status {
    let character = get_character_data(&user.name);

    let body_color = match &patch.bodyColor {
        Some(val) => val.to_owned(),
        None => character.bodyColor
    };

    let hat_id = match &patch.hatId {
        Some(val) => val.to_owned(),
        None => character.hatId
    };

    let face_id = match &patch.faceId {
        Some(val) => val.to_owned(),
        None => character.faceId
    };

    let shirt_id = match &patch.shirtId {
        Some(val) => val.to_owned(),
        None => character.shirtId
    };

    let pants_id = match &patch.pantsId {
        Some(val) => val.to_owned(),
        None => character.pantsId
    };

    let patched_character = Character {
        bodyColor: body_color,
        hatId: hat_id,
        faceId: face_id,
        shirtId: shirt_id,
        pantsId: pants_id
    };

    let patched_string = serde_json::to_string(&patched_character).unwrap();

    use crate::schema::users;
    diesel::update(users::table.filter(users::username.eq(user.name)))
        .set(users::charData.eq(patched_string))
        .execute(&crate::database_connection())
        .expect("Database error");

    Status::Ok
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    bodyColor: Option<String>,
    hatId: Option<String>,
    faceId: Option<String>,
    shirtId: Option<String>,
    pantsId: Option<String>
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct CharacterPatch {
    #[serde(default, deserialize_with = "deserialize_some")]
    bodyColor: Option<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_some")]
    hatId: Option<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_some")]
    faceId: Option<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_some")]
    shirtId: Option<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_some")]
    pantsId: Option<Option<String>>
}

// see https://github.com/serde-rs/serde/issues/984#issuecomment-314143738
// (We need this, because serde_json doesn't differ between null and undefined.)
fn deserialize_some<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where T: Deserialize<'de>,
          D: Deserializer<'de>
{
    Deserialize::deserialize(deserializer).map(Some)
}

fn get_character_data(user: &str) -> Character {
    use crate::schema::users;
    let character = users::table.filter(users::username.eq(user))
        .select(users::charData)
        .first::<String>(&crate::database_connection())
        .expect("Database error");
    serde_json::from_str(&character).unwrap()
}