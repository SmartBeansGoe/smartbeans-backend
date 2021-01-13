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
pub fn post_character(user: guards::User, data: Json<CharacterJson>) -> Result<Status, Status> {
    // TODO: Race Condition?
    use crate::schema::characters::dsl::*;
    let conn = crate::database::establish_connection();

    // If someone tries to write a locked asset, return 403
    let unlocked = unlocked_assets(&user)?;
    if [&data.hat_id, &data.face_id, &data.shirt_id, &data.pants_id].iter().any(
        |id| id.is_some() && unlocked.contains(&id.as_ref().unwrap()
        )) {
        return Err(Status::Unauthorized);
    }

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
    let unlocked_assets = unlocked_assets(&user)?;
    let assets = assets_from_datafile().into_iter()
        .filter(|asset| unlocked_assets.contains(&serde_json::to_string(&asset["asset_id"]).unwrap()));

    let hats = assets.clone()
        .filter(|asset| asset["category"] == Value::String("hats".to_string()))
        .collect::<Vec<Value>>();
    let faces = assets.clone()
        .filter(|asset| asset["category"] == Value::String("faces".to_string()))
        .collect::<Vec<Value>>();
    let shirts = assets.clone()
        .filter(|asset| asset["category"] == Value::String("shirts".to_string()))
        .collect::<Vec<Value>>();
    let pants = assets.clone()
        .filter(|asset| asset["category"] == Value::String("pants".to_string()))
        .collect::<Vec<Value>>();

    let mut all = serde_json::map::Map::new();
    all.insert("hats".to_string(), serde_json::to_value(hats).unwrap());
    all.insert("faces".to_string(), serde_json::to_value(faces).unwrap());
    all.insert("shirts".to_string(), serde_json::to_value(shirts).unwrap());
    all.insert("pants".to_string(), serde_json::to_value(pants).unwrap());

    Ok(Json(Value::Object(all)))
}

pub fn unlocked_assets(user: &crate::guards::User) -> Result<Vec<String>, Status> {
    Ok(assets_from_datafile().iter()
        .filter(|asset| {
            if asset["precondition"] == Value::Null {
                return true;
            }

            let precondition = asset["precondition"].as_str().unwrap();
            let split: Vec<&str> = precondition.split_whitespace().collect();
            let id = split[1].parse::<i64>().unwrap();

            match split[0] {
                "achievement" => {
                    crate::achievements::completed_achievements(&user.name).contains(&id)
                }
                "task" => {
                    crate::smartape::progress(&user.token).unwrap().contains(&id)
                }
                _ => true
            }
        })
        .map(|asset| serde_json::to_string(&asset["asset_id"].clone()).unwrap())
        .collect())
}

fn assets_from_datafile() -> Vec<Value> {
    serde_json::from_str::<Value>(
        &std::fs::read_to_string("data/assets.json").unwrap()
    ).unwrap().as_array().unwrap().clone()
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

    diesel::update(charnames.find(&user.name))
        .set(charname.eq(crate::data_to_string(data)))
        .execute(&crate::database::establish_connection())
        .expect("Database error");

    crate::achievements::AchievementTrigger::new(&user).unwrap().run("nickname_changed");

    Status::Ok
}