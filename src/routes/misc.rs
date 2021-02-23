use rocket_contrib::json::Json;
use serde_json::{Value, Number};
use rocket::http::Status;
use rand::Rng;
use crate::guards;
use diesel::prelude::*;

/// Another example
#[get("/rand/<min>/<max>")]
pub fn rand(min: i32, max: i32, _user: guards::User) -> Json<Value> {
    let rand_num = rand::thread_rng().gen_range(min, max);
    Json(json!({
        "rand": rand_num
    }))
}

/// Returns an error
#[get("/error/<code>")]
pub fn error(code: u16) -> Status {
    Status::from_code(code).unwrap_or(Status::NotFound)
}

#[post("/report_error", format = "text/plain", data = "<data>")]
pub fn report_error(_user: guards::User, data: rocket::Data) -> Status {
    use crate::schema::error_reports::dsl::*;

    let data = crate::data_to_string(data);

    diesel::insert_into(error_reports)
        .values((
            username.eq("see message"),
            time.eq(crate::epoch()),
            message.eq(&data)
        ))
        .execute(&crate::database::establish_connection())
        .expect("Database error");

    crate::bot::report_error(&data);

    Status::Ok
}

#[get("/leaderboard")]
pub fn leaderboard(_user: guards::User) -> Json<Value> {
    use crate::schema::users::dsl::*;

    let mut scores: Vec<(String, String, i64)> = users.select((username, total_score))
        .load::<(String, i64)>(&crate::database::establish_connection())
        .expect("Database error")
        .into_iter()
        .map(|(user, score)| {
            use crate::schema::charnames::dsl::*;

            let name: String = charnames.filter(username.eq(&user))
                .select(charname)
                .first(&crate::database::establish_connection())
                .expect("Database error");

            (user, name, score)
        })
        .collect::<Vec<_>>();

    scores.sort_unstable_by(|a, b| b.2.cmp(&a.2));

    let mut buckets: Vec<Vec<(String, String, i64, usize)>> = Vec::new();
    for (user, name, points) in scores {
        let rank = buckets.len();

        if buckets.is_empty() || buckets[rank-1][0].2 != points {
            buckets.push(vec![(user, name, points, rank+1)]);
            continue;
        }

        buckets[rank-1].push((user, name, points, rank));
    }

    let values: Vec<Value> = buckets[..std::cmp::min(30, buckets.len())].concat()
        .iter()
        .map(|data| {
            let mut v = json!({});
            v["charname"] = Value::String(data.1.clone());
            v["score"] = Value::Number(Number::from(data.2));
            v["rank"] = Value::Number(Number::from(data.3));
            v["character"] = serde_json::to_value(
                crate::routes::character::character_information(&data.0)
            ).unwrap();
            v["character"]["username"] = Value::String("redacted".to_string());

            v
        })
        .collect();

    Json(serde_json::to_value(values).unwrap())
}