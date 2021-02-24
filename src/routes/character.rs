use rocket::http::Status;
use rocket_contrib::json::Json;
use serde_json::Value;
use diesel::prelude::*;

use crate::guards;
use crate::models::Character;
use crate::static_data::ASSETS;

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
pub fn post_character(user: guards::User, data: Json<CharacterJson>) -> Result<Status, Status> {
    let conn = crate::database::establish_connection();

    // If someone tries to write a locked asset, return 403
    let unlocked = unlocked_assets(&user)?;
    if [&data.hat_id, &data.face_id, &data.shirt_id, &data.pants_id].iter().any(
        |id| id.is_some() && !unlocked.contains(&id.as_ref().unwrap()
        )) {
        return Err(Status::Unauthorized);
    }

    // Update asset data in database
    {
        use crate::schema::characters::dsl::*;

        // Because we rewrite every value anyway, we can just delete and recreate the dataset
        diesel::delete(characters.filter(username.eq(&user.name)))
            .execute(&conn)
            .expect("Database error");

        diesel::insert_into(characters)
            .values((
                username.eq(&user.name),
                body_color.eq(&data.body_color),
                hat_id.eq(&data.hat_id),
                face_id.eq(&data.face_id),
                shirt_id.eq(&data.shirt_id),
                pants_id.eq(&data.pants_id)
            ))
            .execute(&conn)
            .expect("Database error");
    }

    // Update counter in database (for achievement)
    {
        use crate::schema::users::dsl::*;
        let counter: i64 = users.filter(username.eq(&user.name))
            .select(char_changed)
            .first(&conn)
            .expect("Database error");

        diesel::update(users.filter(username.eq(&user.name)))
            .set(char_changed.eq(counter + 1))
            .execute(&conn)
            .expect("Database error");
    }

    crate::log_route(&user.name, "POST /character", Some(&format!("{:?}", data)));

    crate::achievements::AchievementTrigger::new(&user)?.run("char_changed");

    Ok(Status::Ok)
}

pub fn character_information(user: &str) -> Option<Character> {
    use crate::schema::characters::dsl::*;

    characters.filter(username.eq(user))
        .first(&crate::database::establish_connection())
        .ok()
}

#[get("/assets")]
pub fn get_assets(user: guards::User) -> Result<Json<Value>, Status> {
    Ok(Json(serde_json::to_value(unlocked_assets(&user)?).unwrap()))
}

pub fn unlocked_assets(user: &crate::guards::User) -> Result<Vec<String>, Status> {
    let progress = crate::smartape::progress(&user.token).unwrap();
    let achievements = crate::achievements::completed_achievements(&user.name);

    Ok(ASSETS.iter()
        .filter(|asset| {
            let precond_task = Value::as_i64(&asset["precondition"]["task-id"]);
            let precond_achievement = Value::as_i64(&asset["precondition"]["achievement-id"]);

            match (precond_task, precond_achievement) {
                (None, None) => true,
                (Some(id), _) => progress.contains(&id),
                (None, Some(id)) => achievements.contains(&id)
            }
        })
        .map(|asset| Value::as_str(&asset["id"]).unwrap().to_string())
        .collect())
}

#[get("/charname")]
pub fn get_charname(user: guards::User) -> Json<Value> {
    use crate::schema::charnames::dsl::*;

    let name: String = charnames.filter(username.eq(&user.name))
        .select(charname)
        .first(&crate::database::establish_connection())
        .expect("Database error");

    Json(json!({
        "charname": name
    }))
}

#[post("/charname", data = "<data>", format = "text/plain")]
pub fn post_charname(user: guards::User, data: rocket::Data) -> Status {
    use crate::schema::charnames::dsl::*;

    let new_name = crate::data_to_string(data);
    diesel::update(charnames.find(&user.name))
        .set(charname.eq(&new_name))
        .execute(&crate::database::establish_connection())
        .expect("Database error");

    crate::log_route(&user.name, "POST /charname", Some(&new_name));
    crate::achievements::AchievementTrigger::new(&user).unwrap().run("nickname_changed");

    Status::Ok
}