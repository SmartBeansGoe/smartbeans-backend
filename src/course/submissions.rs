use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde_json::Value;
use rand::seq::SliceRandom;
use crate::auth::guards;
use crate::schema::submissions;
use crate::SETTINGS;
use reqwest::header::CONTENT_TYPE;

#[get("/courses/<course>/tasks/all/submissions", rank = 1)]
pub fn route_get_all_submissions(user: guards::User, course: String) -> Result<Json<Vec<PublicSubmission>>, Status> {
    if course != user.course {
        return Err(Status::Forbidden);
    }

    Ok(Json(get_public_submissions(&user.name, &course)))
}

#[get("/courses/<course>/tasks/<taskid>/submissions", rank = 2)]
pub fn route_get_task_submissions(user: guards::User, course: String, taskid: i32) -> Result<Json<Vec<PublicSubmission>>, Status> {
    if course != user.course {
        return Err(Status::Forbidden);
    }

    let submissions = get_public_submissions(&user.name, &course)
        .into_iter()
        .filter(|sub| sub.taskid == taskid)
        .collect::<Vec<_>>();

    Ok(Json(submissions))
}

#[get("/courses/<course>/tasks/<taskid>/submissions/<submissionid>")]
pub fn route_get_single_submission(user: guards::User, course: String, taskid: i32, submissionid: i32) -> Result<Json<PublicSubmission>, Status> {
    if course != user.course {
        return Err(Status::Forbidden);
    }

    let submission = get_public_submissions(&user.name, &course)
        .into_iter()
        .filter(|sub| sub.taskid == taskid)
        .filter(|sub| sub.id == submissionid)
        .next();

    Ok(Json(submission.ok_or(Status::NotFound)?))
}

#[post("/courses/<course>/tasks/<taskid>/submissions", data = "<data>")]
pub async fn route_post_submission(user: guards::User, course: String, taskid: i32, data: Json<Value>) -> Result<Json<Value>, Status> {
    if course != user.course {
        return Err(Status::Forbidden);
    }

    let submission = data["submission"].as_str()
        .ok_or(Status::BadRequest)?;

    use crate::schema::tasks;
    let (lang, tests) = tasks::table.filter(tasks::taskid.eq(taskid))
        .select((tasks::lang, tasks::tests))
        .first::<(String, String)>(&crate::database_connection())
        .expect("Database error");

    let result = submit_solution(taskid,&lang, &serde_json::from_str(&tests).unwrap(), submission).await;

    use crate::schema::submissions;
    diesel::insert_into(submissions::table)
        .values((
            submissions::user.eq(user.name),
            submissions::course.eq(course),
            submissions::taskid.eq(taskid),
            submissions::timestamp.eq(crate::tools::epoch()),
            submissions::content.eq(submission),
            submissions::resultType.eq(result["type"].as_str().unwrap()),
            submissions::simplified.eq(serde_json::to_string(&result["simplified"]).unwrap()),
            submissions::details.eq(serde_json::to_string(&result["details"]).unwrap()),
            submissions::score.eq(result["score"].as_f64().unwrap() as f32)
        ))
        .execute(&crate::database_connection())
        .expect("Database error");

    Ok(Json(json!({
        "type": result["type"].as_str().unwrap(),
        "score": result["score"].as_f64().unwrap()
    })))
}

#[derive(Debug, Deserialize, Queryable)]
struct Submission {
    id: i32,
    user: String,
    course: String,
    taskid: i32,
    timestamp: i64,
    content: String,
    result_type: String,
    simplified: String,
    details: String,
    score: f32
}

#[derive(Serialize)]
pub struct PublicSubmission {
    id: i32,
    taskid: i32,
    timestamp: i64,
    content: String,
    result_type: String,
    simplified: Value,
    score: f32
}

fn get_public_submissions(user: &str, course: &str) -> Vec<PublicSubmission> {
    submissions::table.filter(submissions::user.eq(user))
        .filter(submissions::course.eq(course))
        .load::<Submission>(&crate::database_connection())
        .expect("Database error")
        .into_iter()
        .map(|sub| {
            PublicSubmission {
                id: sub.id,
                taskid: sub.taskid,
                timestamp: sub.timestamp,
                content: sub.content,
                result_type: sub.result_type,
                simplified: serde_json::from_str(&sub.simplified).unwrap(),
                score: sub.score
            }
        })
        .collect::<Vec<_>>()
}

pub async fn submit_solution(taskid: i32, lang: &str, tests: &Value, submission: &str) -> Value {
    let sandbox = SETTINGS.get::<Vec<String>>("sandbox.urls")
        .expect("sandbox.urls missing in settings")
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string();

    let body = json!({
        "taskid": taskid,
        "submission": submission,
        "lang": lang,
        "tests": tests
    });

    reqwest::Client::new()
        .post(&format!("{}/evaluate", sandbox))
        .header(CONTENT_TYPE, "application/json")
        .json(&body)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}