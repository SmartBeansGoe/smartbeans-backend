//! Module for points, skills and levels

use cached::proc_macro::cached;
use rocket::http::Status;
use serde_json::Value;
use diesel::prelude::*;
use std::collections::HashMap;
use crate::static_data::TASK_STATS;

// based on (x-1)^1.75 * 15
pub static LEVELS: [u64; 14] = [100000000, 0, 15, 50, 100, 170, 250, 340, 450, 570, 700, 840, 1080, 1400];

#[cached(time = 3600)]
pub fn total_points() -> HashMap<String, u64> {
    let mut map = HashMap::new();

    for task in TASK_STATS.values() {
        *map.entry("total".to_string()).or_insert(0) += task["points"].as_u64().unwrap();

        for skill in task["skills"].as_array().unwrap() {
            *map.entry(skill["name"].as_str().unwrap().to_string()).or_insert(0) += skill["points"].as_u64().unwrap();
        }
    }

    map
}

pub fn user_points(user: &crate::guards::User) -> Result<HashMap<String, u64>, Status> {
    let mut map = HashMap::new();

    for key in total_points().keys() {
        map.insert(key.to_string(), 0);
    }

    let progress = crate::smartape::progress(&user.token)?;
    let tasks = crate::smartape::tasks(user.token.to_string())?
        .into_iter()
        .filter(|task| task["points"].as_u64().is_some())
        .filter(|task| progress.contains(&task["taskid"].as_i64().unwrap()))
        .collect::<Vec<Value>>();

    for task in tasks {
        *map.entry("total".to_string()).or_insert(0) += task["points"].as_u64().unwrap();

        for skill in task["skills"].as_array().unwrap() {
            *map.entry(skill["name"].as_str().unwrap().to_string()).or_insert(0) += skill["points"].as_u64().unwrap();
        }
    }

    // Store total score in database (for leaderboard)
    use crate::schema::users::dsl::*;
    diesel::update(users.filter(username.eq(&user.name)))
        .set(total_score.eq(map["total"] as i64))
        .execute(&crate::database::establish_connection())
        .expect("Database error");

    Ok(map)
}

pub fn level_to_points(lvl: u64) -> u64 {
    let lvl = lvl as usize;

    if lvl < LEVELS.len() {
        LEVELS[lvl]
    }
    else {
        *LEVELS.last().unwrap()
    }
}

pub fn points_to_level(points: u64) -> u64 {
    for (lvl, min_points) in LEVELS.iter().enumerate().rev() {
        if &points >= min_points {
            return lvl as u64;
        }
    }

    unreachable!()
}