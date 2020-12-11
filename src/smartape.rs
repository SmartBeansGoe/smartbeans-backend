/// This module contains wrappers for several SmartApe routes.
/// See https://gitlab.gwdg.de/lorenz.glimann/smartape-dokumentation/-/wikis/api for more information.

use std::env;

use reqwest::blocking::Response;
use reqwest::header::{COOKIE, CONTENT_TYPE};
use rocket::http::Status;
use serde_json::Value;
use diesel::prelude::*;

/// Wrapper for GET/POST /login/... . Returns a session token. Returns 404 if the user does not
/// exist in the SmartApe database.
pub fn login(username: &str, ltidata: Option<&str>) -> Result<String, Status> {
    let (method, data) = match ltidata {
        Some(d) => ("POST", d),
        None => ("GET", "")
    };

    let data = serde_urlencoded::from_str::<Value>(data).unwrap();
    let data = serde_json::to_string(&data).unwrap();

    Ok(call_smartape_api(
        method,
        &format!("/login/{}?apiKey={}", username, env::var("SMARTAPE_API_KEY").unwrap()),
        None,
        &data,
        true
    )?.cookies()
        .find(|c| c.name() == "connect.sid")
        .unwrap()
        .value()
        .to_string())
}

/// Wrapper for "GET /sessiondata: Hand out Session data to the frontend"
pub fn sessiondata(token: &str) -> Result<Value, Status> {
    let data_str = call_smartape_api(
        "GET",
        "/sessiondata",
        Some(token),
        "",
        false
    )?.text().unwrap();

    Ok(serde_json::from_str(&data_str).unwrap())
}

/// Wrapper for POST /course/:courseid/progresses: Returns a vector of all solved tasks
pub fn progress(token: &str) -> Result<Vec<i64>, Status> {
    let response = call_smartape_api(
        "GET",
        &format!("/course/{}/progresses", courseid(&token)?),
        Some(token),
        "",
        false
    )?.text().unwrap();

    if response.len() <= 2 {
        return Ok(Vec::new());
    }

    Ok(String::from(&response[1..response.len()-1])
        .split(',')
        .map(|id| id.parse::<i64>().unwrap())
        .collect())
}

/// Wrapper for "GET /course/:courseid/tasks: Get a list of all tasks"
/// Also contains information whether a task is solved.
pub fn tasks(token: &str) -> Result<Value, Status> {
    let tasks_str = call_smartape_api(
        "GET",
        &format!("/course/{}/tasks", courseid(&token)?),
        Some(token),
        "",
        false
    )?.text().unwrap();

    // Repack data to add "solved" item
    let tasks: Value = serde_json::from_str(&tasks_str).unwrap();
    let solved = progress(token)?;

    let tasks_vec = tasks.as_array()
        .unwrap()
        .iter()
        .map(|task| {
            serde_json::from_str::<Value>(
                &format!("{{\
                    \"name\":{},\
                    \"shortname\":{},\
                    \"task\":{},\
                    \"taskid\":{},\
                    \"solved\":{}\
                }}",
                task["name"],
                task["shortname"],
                task["task"],
                task["taskid"],
                solved.contains(&serde_json::to_string(&task["taskid"]).unwrap().parse().unwrap()))
            ).unwrap()
        })
        .collect::<Vec<_>>();

    Ok(serde_json::to_value(tasks_vec).unwrap())
}

/// Wrapper for "GET /course/:courseid/tasks/:taskid/submissions: Get a list of all submission atempts"
pub fn submissions(token: &str, taskid: &str) -> Result<Value, Status> {
    let submissions_str = call_smartape_api(
        "GET",
        &format!("/course/{}/tasks/{}/submissions", courseid(&token)?, taskid),
        Some(token),
        "",
        false
    )?.text().unwrap();

    Ok(serde_json::from_str(&submissions_str).unwrap())
}

/// Wrapper for "GET /course/:courseid/tasks/:taskid/submissions/:timestamp: Get details for a submission, including results"
pub fn submission(token: &str, taskid: &str, timestamp: &str) -> Result<Value, Status> {
    let submission_str = call_smartape_api(
        "GET",
        &format!("/course/{}/tasks/{}/submissions/{}", courseid(&token)?, taskid, timestamp),
        Some(token),
        "",
        false
    )?.text().unwrap();

    Ok(serde_json::from_str(&submission_str).unwrap())
}

/// Wrapper for "POST /course/:courseid/tasks/:taskid/submissions: Submit a task"
/// Currently unimplemented
pub fn submit() {
    // TODO
    unimplemented!()
}

/// Wrapper for "POST /share: Share a task i.e. create a pad"
/// Currently unimplemented
pub fn share() {
    // TODO
    unimplemented!()
}

/// Calls the SmartApe backend API (URL specified in .env). Returns 500 if something went wrong.
///
/// * method: Either "GET" or "POST"
/// * route: API route to call (relative to the backend URL)
/// * session_token: SmartBeans auth token (gets automatically converted to SmartApe token)
/// * body: Request body (POST only; use "" if you want to leave it empty)
/// * json: Send body data as JSON?
fn call_smartape_api(method: &str, route: &str, session_token: Option<&str>, body: &str, json: bool) -> Result<Response, Status> {
    let session_token = smartape_token(session_token)?;
    let client = reqwest::blocking::Client::new();
    let url = format!("{}{}", env::var("SMARTAPE_URL").unwrap(), route);
    let auth_cookie = format!("connect.sid={}", session_token);

    let request = match method {
        "GET" => {
            client.get(&url)
        }
        "POST" => {
            if json {
                client.post(&url).header(CONTENT_TYPE, "application/json").body(body.to_string())
            }
            else {
                client.post(&url).body(body.to_string())
            }
        }
        _ => panic!("Invalid method: {}", method)
    };

    let response = request.header(COOKIE, auth_cookie).send().unwrap();

    if response.status().is_success() {
        Ok(response)
    }
    else if response.status().as_u16() == 404 {
        Err(Status::NotFound)
    }
    else {
        // As far as I can see, SmartApe almost always returns 500 when something went wrong
        Err(Status::InternalServerError)
    }
}

/// Retrieves the SmartApe session token for a SmartBeans auth token.
/// Returns "0" if token is None.
fn smartape_token(token: Option<&str>) -> Result<String, Status> {
    if token.is_none() {
        return Ok("0".to_string());
    }
    let token = token.unwrap();

    use crate::schema::sessions::dsl::*;

    let conn = diesel::sqlite::SqliteConnection::establish("db.sqlite").unwrap();

    if let Ok(ape_token) = sessions.filter(auth_token.eq(token))
        .select(smartape_token)
        .first(&conn) {
        Ok(ape_token)
    } else {
        Err(Status::Unauthorized)
    }
}

/// Returns the course id corresponding to the session
fn courseid(session_token: &str) -> Result<String, Status> {
    Ok(
        sessiondata(session_token)?["courseid"]
            .as_str()
            .unwrap()
            .to_string()
    )
}