use diesel::prelude::*;
use std::collections::HashMap;
use rocket::serde::json::Json;
use serde_json::Value;
use rocket::http::Status;
use crate::course::name_to_title;
use crate::auth::guards;
use crate::schema::{tasks, courseTask};

#[get("/courses/<course>/tasks")]
pub fn route_get_tasks(course: String) -> Result<Json<Vec<PublicTask>>, Status> {
    if name_to_title(&course).is_none() {
        return Err(Status::NotFound);
    }

    Ok(Json(get_course_tasks(&course)))
}

#[get("/courses/<course>/tasks/<taskid>")]
pub fn route_get_single_task(course: String, taskid: i32) -> Result<Json<PublicTask>, Status> {
    if name_to_title(&course).is_none() {
        return Err(Status::NotFound);
    }

    let task = get_course_tasks(&course).into_iter()
        .filter(|task| task.taskid == taskid)
        .next()
        .ok_or(Status::NotFound)?;

    Ok(Json(task))
}

#[post("/task", data = "<data>")]
pub fn route_post_task(_key: guards::AdminKey, data: Json<Value>) -> Result<Status, Status> {
    let task = Task {
        taskid: data["taskid"].as_i64().ok_or(Status::BadRequest)? as i32,
        task_description: serde_json::to_string(&data["taskDescription"]).unwrap(),
        solution: data["solution"].as_str().ok_or(Status::BadRequest)?.to_string(),
        lang: data["lang"].as_str().ok_or(Status::BadRequest)?.to_string(),
        tests: serde_json::to_string(&data["tests"]).unwrap()
    };

    let meta  = data["courseMetaData"].as_array()
        .ok_or(Status::BadRequest)?
        .into_iter()
        .map(|val| {
            Ok(Mapping {
                course: val["courseName"].as_str().ok_or(Status::BadRequest)?.to_string(),
                taskid: data["taskid"].as_i64().ok_or(Status::BadRequest)? as i32,
                tags: serde_json::to_string(&val["tags"]).unwrap(),
                order_by: val["orderBy"].as_i64().ok_or(Status::BadRequest)? as i32,
                prerequisites: serde_json::to_string(&val["prerequisites"]).unwrap(),
            }) as Result<Mapping, Status>
        })
        .collect::<Result<Vec<_>, _>>()?;

    diesel::insert_into(tasks::table)
        .values(task)
        .execute(&crate::database_connection())
        .expect("Database error");

    diesel::insert_into(courseTask::table)
        .values(meta)
        .execute(&crate::database_connection())
        .expect("Database error");

    Ok(Status::Ok)
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
struct Task {
    taskid: i32,
    #[column_name = "taskDescription"]
    task_description: String,
    solution: String,
    lang: String,
    tests: String
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "courseTask"]
struct Mapping {
    course: String,
    taskid: i32,
    tags: String,
    #[column_name = "orderBy"]
    order_by: i32,
    prerequisites: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicTask {
    taskid: i32,
    task_description: Value,
    lang: String,
    tags: Value,
    order_by: i32,
    prerequisites: Value
}

fn get_all_tasks() -> Vec<Task> {
    tasks::table.load::<Task>(&crate::database_connection())
        .expect("Database error")
}

fn get_course_tasks(course: &str) -> Vec<PublicTask> {
    let mut mapping = courseTask::table.filter(courseTask::course.eq(course))
        .load::<Mapping>(&crate::database_connection())
        .expect("Database error")
        .into_iter()
        .fold(HashMap::new(), |mut acc, elem| {
            acc.insert(elem.taskid, elem);
            acc
        });

    let taskids = mapping.keys().map(|key| key.to_owned()).collect::<Vec<_>>();
    get_all_tasks().into_iter()
        .filter(|task| taskids.contains(&&task.taskid))
        .map(|task| {
            let map = mapping.remove(&task.taskid).unwrap();

            PublicTask {
                taskid: task.taskid,
                task_description: serde_json::from_str(&task.task_description).unwrap(),
                lang: task.lang,
                tags: serde_json::from_str(&map.tags).unwrap(),
                order_by: map.order_by,
                prerequisites: serde_json::from_str(&map.prerequisites).unwrap()
            }
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("{:#?}", super::get_course_tasks("testbeans"));
    }
}