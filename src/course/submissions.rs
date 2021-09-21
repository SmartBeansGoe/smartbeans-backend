use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde_json::Value;
use crate::auth::guards;
use crate::schema::submissions;

#[get("/course/<course>/tasks/all/submissions", rank = 1)]
pub fn route_get_all_submissions(user: guards::User, course: String) -> Result<Json<Vec<PublicSubmission>>, Status> {
    if course != user.course {
        return Err(Status::Forbidden);
    }

    Ok(Json(get_public_submissions(&user.name, &course)))
}

#[get("/course/<course>/tasks/<taskid>/submissions", rank = 2)]
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

#[get("/course/<course>/tasks/<taskid>/submissions/<submissionid>")]
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