use diesel::prelude::*;
use std::collections::HashMap;
use rocket::serde::json::Json;
use rocket::http::Status;
use crate::course::exists;

#[get("/courses/<course>/tasks")]
pub fn route_get_tasks(course: String) -> Result<Json<Vec<PublicTask>>, Status> {
    if !exists(&course) {
        return Err(Status::NotFound);
    }

    Ok(Json(get_course_tasks(&course)))
}

#[get("/courses/<course>/tasks/<taskid>")]
pub fn route_get_single_task(course: String, taskid: i32) -> Result<Json<PublicTask>, Status> {
    if !exists(&course) {
        return Err(Status::NotFound);
    }

    let task = get_course_tasks(&course).into_iter()
        .filter(|task| task.taskid == taskid)
        .next()
        .ok_or(Status::NotFound)?;

    Ok(Json(task))
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
struct Task {
    taskid: i32,
    task_description: String,
    solution: String,
    lang: String,
    tests: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicTask {
    taskid: i32,
    task_description: String,
    lang: String,
    tags: String,
    order_by: i32,
    prerequisites: String
}

fn get_all_tasks() -> Vec<Task> {
    use crate::schema::tasks;
    tasks::table.load::<Task>(&crate::database_connection())
        .expect("Database error")
}

fn get_course_tasks(course: &str) -> Vec<PublicTask> {
    #[derive(Debug, Serialize, Deserialize, Queryable)]
    struct Mapping {
        course: String,
        taskid: i32,
        tags: String,
        order_by: i32,
        prerequisites: String
    }

    use crate::schema::courseTask;
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
                task_description: task.task_description,
                lang: task.lang,
                tags: map.tags,
                order_by: map.order_by,
                prerequisites: map.prerequisites
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