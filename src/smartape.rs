/// This module contains wrappers for several SmartApe routes.
/// See https://gitlab.gwdg.de/lorenz.glimann/smartape-dokumentation/-/wikis/api for more information.

use std::env;

use reqwest::blocking::Response;
use reqwest::header::COOKIE;
use rocket::http::Status;
use serde_json::Value;

/// Wrapper for "GET /sessiondata: Hand out Session data to the frontend"
/// Result string can be interpreted as valid JSON.
pub fn sessiondata(token: &str) -> Result<String, Status> {
    Ok(call_smartape_api(
        "GET",
        "/sessiondata",
        token,
        ""
    )?.text().unwrap())
}

/// Wrapper for "POST /course/:courseid/progress: Get a users progress"
pub fn progress(token: &str) -> Result<i64, Status> {
    let response = call_smartape_api(
        "GET",
        &format!("/course/{}/progress", courseid(&token)?),
        token,
        ""
    )?;

    Ok(response.text().unwrap().parse().unwrap())
}

/// Wrapper for "GET /course/:courseid/tasks: Get a list of all tasks"
/// Result string can be interpreted as valid JSON.
pub fn tasks(token: &str) -> Result<String, Status> {
    Ok(call_smartape_api(
        "GET",
        &format!("/course/{}/tasks", courseid(&token)?),
        token,
        ""
    )?.text().unwrap())
}

/// Wrapper for "GET /course/:courseid/tasks/:taskid: Get a detailed description of a task"
/// Result string can be interpreted as valid JSON.
pub fn task(token: &str, taskid: &str) -> Result<String, Status> {
    Ok(call_smartape_api(
        "GET",
        &format!("/course/{}/tasks/{}", courseid(&token)?, taskid),
        token,
        ""
    )?.text().unwrap())
}

/// Wrapper for "GET /course/:courseid/tasks/:taskid/submissions: Get a list of all submission atempts"
/// Result string can be interpreted as valid JSON.
pub fn submissions(token: &str, taskid: &str) -> Result<String, Status> {
    Ok(call_smartape_api(
        "GET",
        &format!("/course/{}/tasks/{}/submissions", courseid(&token)?, taskid),
        token,
        ""
    )?.text().unwrap())
}

/// Wrapper for "GET /course/:courseid/tasks/:taskid/submissions/:timestamp: Get details for a submission, including results"
/// Result string can be interpreted as valid JSON.
pub fn submission(token: &str, taskid: &str, timestamp: &str) -> Result<String, Status> {
    Ok(call_smartape_api(
        "GET",
        &format!("/course/{}/tasks/{}/submissions/{}", courseid(&token)?, taskid, timestamp),
        token,
        ""
    )?.text().unwrap())
}

/// Wrapper for "POST /course/:courseid/tasks/:taskid/submissions: Submit a task"
/// Currently unimplemented
pub fn submit() {
    // TODO
    unimplemented!()
}

/// Wrapper for "GET /listshares: List all shared tasks"
/// Result string can be interpreted as valid JSON.
/// Currently, this returns 404 from SmartApe. Further investigation needed. (TODO)
pub fn listshares(token: &str) -> Result<String, Status> {
    Ok(call_smartape_api(
        "GET",
        "/listshares",
        token,
        ""
    )?.text().unwrap())
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
/// * session_token: token from connect.sid cookie
/// * body: Request body (POST only; use "" if you want to leave it empty)
fn call_smartape_api(method: &str, route: &str, session_token: &str, body: &str) -> Result<Response, Status> {
    let client = reqwest::blocking::Client::new();
    let url = format!("{}{}", env::var("SMARTAPE_URL").unwrap(), route);
    let auth_cookie = format!("connect.sid={}", session_token);

    let request = match method {
        "GET" => {
            client.get(&url)
        }
        "POST" => {
            client.post(&url)
                .body(body.to_string())
        }
        _ => panic!("Invalid method: {}", method)
    };

    let response = request.header(COOKIE, auth_cookie).send().unwrap();

    if response.status().is_success() {
        Ok(response)
    }
    else {
        // As far as I can see, SmartApe always returns 500 when something went wrong
        Err(Status::InternalServerError)
    }
}

/// Returns the course id corresponding to the session
fn courseid(session_token: &str) -> Result<String, Status> {
    let response = sessiondata(session_token)?;
    let response_json: Value = serde_json::from_str(&response).unwrap();

    Ok(response_json["courseid"].as_str().unwrap().to_string())
}