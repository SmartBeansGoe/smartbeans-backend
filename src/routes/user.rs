use rocket_contrib::json::Json;
use rocket::http::Status;
use serde_json::{Value, Number};
use diesel::prelude::*;
use crate::guards;
use crate::level;

/// Returns the username
#[get("/username")]
pub fn get_username(user: guards::User) -> Json<Value> {
    Json(json!({
        "username": user.name
    }))
}

/// Returns some userdata
#[get("/user/data")]
pub fn get_userdata(user: guards::User) -> Json<Value> {
    use crate::schema::users::dsl::*;
    let conn = crate::database::establish_connection();

    let the_first_login_that_is_not_the_table_column: bool = users
        .filter(username.eq(&user.name))
        .select(first_login)
        .first(&conn)
        .expect("Database error");

    let survey_completed = users.filter(username.eq(&user.name))
        .select(survey)
        .first::<Option<String>>(&conn)
        .expect("Database error")
        .is_some();

    Json(json!({
        "username": user.name,
        "first_login": the_first_login_that_is_not_the_table_column,
        "survey_completed": survey_completed
    }))
}

/// Sets first_login to false
#[post("/user/first_login_done")]
pub fn first_login_done(user: guards::User) -> Status {
    use crate::schema::users::dsl::*;

    diesel::update(users.filter(username.eq(&user.name)))
        .set(first_login.eq(false))
        .execute(&crate::database::establish_connection())
        .expect("Database error");

    Status::Ok
}

#[post("/user/submit_survey", format = "text/plain", data = "<data>")]
pub fn submit_survey(user: guards::User, data: rocket::Data) -> Status {
    use crate::schema::users::dsl::*;

    diesel::update(users.filter(username.eq(&user.name)))
        .set(survey.eq(crate::data_to_string(data)))
        .execute(&crate::database::establish_connection())
        .expect("Database error");

    Status::Ok
}

#[get("/level_data")]
pub fn level_data(user: guards::User) -> Result<Json<Value>, Status> {
    let userdata = level::user_points(&user)?;
    let maxdata = level::total_points();
    let level = level::points_to_level(userdata["total"]);

    let mut result = json!({});

    result["level"] = Value::Number(Number::from(level));
    result["points"] = Value::Number(Number::from(userdata["total"]));
    result["next_points"] = Value::Number(Number::from(level::level_to_points(level + 1)));
    result["max_level"] = Value::Number(Number::from(level::LEVELS.len() - 1));

    let mut skills = Vec::new();
    for (name, max_points) in maxdata.into_iter() {
        if name == "total" {
            continue;
        }

        let mut skill = json!({});
        skill["points"] = Value::Number(Number::from(userdata[&name]));
        skill["max_points"] = Value::Number(Number::from(max_points));
        skill["name"] = Value::String(name);
        skills.push(skill);
    }

    result["skills"] = serde_json::to_value(skills).unwrap();

    Ok(Json(result))
}

/// Return all achievements for the user
#[get("/achievements")]
pub fn get_achievements(user: guards::User) -> Json<Value> {
    Json(serde_json::to_value(crate::achievements::achievements(&user.name)).unwrap())
}

#[post("/achievements/404")]
pub fn set_404(user: guards::User) -> Result<Status, Status> {
    crate::achievements::AchievementTrigger::run(&user, "404")?;

    Ok(Status::Ok)
}

#[get("/system_messages")]
pub fn system_messages(user: guards::User) -> Json<Value> {
    let messages = crate::system_messages::receive_messages(&user.name);
    let mut messages_json = Vec::new();

    for message in messages {
        let mut message_json = json!({});
        message_json["id"] = Value::Number(Number::from(message.id));
        message_json["timestamp"] = Value::Number(Number::from(message.time));
        message_json["content"] = match &message.message_type[..] {
            "achievement_unlocked" => serde_json::from_str(&message.content).unwrap(),
            _ => Value::String(message.content)
        };
        message_json["type"] = Value::String(message.message_type);

        messages_json.push(message_json);
    }

    Json(serde_json::to_value(messages_json).unwrap())
}