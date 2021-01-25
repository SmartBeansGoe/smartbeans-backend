use rocket::http::Status;
use serde_json::Value;
use diesel::prelude::*;
use diesel::insert_into;
//use cached::proc_macro::cached;

use std::collections::HashMap;

use crate::static_data::ACHIEVEMENTS;

pub struct AchievementTrigger {
    /// User name
    username: String,
    /// Token
    token: String,
    /// All achievements as JSON values (see data/achievements.json)
    achievements: &'static Vec<Value>,
    /// A list with the ids of completed achievements
    completed: Vec<i64>,
    /// A list with all completed tasks
    solved_tasks: Vec<i64>,
    /// All submissions as JSON values
    submissions: Vec<Value>,
}

impl AchievementTrigger {
    pub fn new(user: &crate::guards::User) -> Result<AchievementTrigger, Status> {
        let submissions = crate::smartape::all_submissions(&user.token)?;

        Ok(AchievementTrigger {
            username: String::from(&user.name),
            token: String::from(&user.token),
            achievements: &ACHIEVEMENTS,
            completed: completed_achievements(&user.name),
            solved_tasks: crate::smartape::progress(&user.token)?,
            submissions
        })
    }

    pub fn run(&self, trigger: &str) {
        // TODO: Run in background thread
        let trigger = Value::String(trigger.to_string());

        let ids = self.achievements.iter()
            .filter(|achievement| {
                !self.completed.contains(&achievement["id"].as_i64().unwrap())
            })
            .filter(|achievement| {
                achievement["triggers"].as_array().unwrap().contains(&trigger)
            })
            .map(|achievement| {
                achievement["id"].as_i64().unwrap()
            })
            .collect::<Vec<_>>();

        for id in ids {
            if self.check(id) {
                set_achievement_completed(&self.username, id);
                let achievement_json = achievements(&self.username)
                    .into_iter()
                    .find(|a| a["id"] == id)
                    .unwrap();

                crate::system_messages::send_message(
                    &self.username,
                    "achievement_unlocked",
                    &achievement_json.to_string()
                )
            }
        }
    }

    fn check(&self, id: i64) -> bool {
        match id {
            1 => self.check_1(),
            2 => self.check_2(),
            3 => self.check_3(),
            4 => self.check_4(),
            5 => self.check_5(),
            6 => self.check_6(),
            _ => panic!("Non-existent achievement id")
        }
    }

    // === Check functions for individual achievements =====

    // Erste Erfolge; login, submission
    fn check_1(&self) -> bool {
        self.solved_tasks.len() >= 3
    }

    // Alles eine Frage des Systems; login, submission
    fn check_2(&self) -> bool {
        self.solved_tasks.len() >= 16
    }

    // Auf alles eine Antwort; login, submission
    fn check_3(&self) -> bool {
        self.solved_tasks.len() >= 42
    }

    // Perfektionist; login, submission
    fn check_4(&self) -> bool {
        self.solved_tasks.len() >= crate::smartape::tasks(self.token.clone()).unwrap().len()
    }

    // Namen sind Schall und Rauch; nickname_changed
    fn check_5(&self) -> bool {
        true
    }

    // Verkleidungskünstler; char_changed
    fn check_6(&self) -> bool {
        let char = crate::routes::character::character_information(&self.username)
            .unwrap();

        char.body_color.is_some()
        && char.hat_id.is_some()
        && char.shirt_id.is_some()
        && char.pants_id.is_some()
    }
}

/// Returns achievements for a user.
/// Return format: [ { "id": ..., "name": ..., "description": ..., "completed": ..., "frequency": ... }, ... ]
pub fn achievements(username: &str) -> Vec<Value> {
    let mut result = Vec::new();
    let frequencies = frequencies();

    for achievement in ACHIEVEMENTS.iter() {
        let mut val = json!({});

        let id = achievement["id"].as_i64().unwrap();
        let completed = get_achievement_completed(username, id);

        val["id"] = achievement["id"].clone();
        val["name"] = achievement["name"].clone();
        val["completed"] = serde_json::to_value(completed).unwrap();
        val["frequency"] = serde_json::to_value(frequencies[&id]).unwrap();
        val["description"] = if completed.is_some() {
            achievement["description"]["completed"].clone()
        } else {
            achievement["description"]["open"].clone()
        };

        if achievement["hidden"].as_bool() == Some(false) || completed.is_some() {
            result.push(val);
        }
    }

    result
}

pub fn completed_achievements(name: &str) -> Vec<i64> {
    use crate::schema::achievements::dsl::*;
    let conn = crate::database::establish_connection();

    achievements.filter(username.eq(name))
        .select(achievementId)
        .load(&conn)
        .expect("Database error")
}

fn get_achievement_completed(uname: &str, achievement_id: i64) -> Option<i64> {
    use crate::schema::achievements::dsl::*;
    let conn = crate::database::establish_connection();

    achievements.filter(username.eq(uname))
        .filter(achievementId.eq(achievement_id))
        .select(completionTime)
        .first(&conn)
        .ok()
}

fn set_achievement_completed(uname: &str, achievement_id: i64) {
    use crate::schema::achievements::dsl::*;
    let conn = crate::database::establish_connection();

    insert_into(achievements).values((
            username.eq(uname),
            achievementId.eq(achievement_id),
            completionTime.eq(crate::epoch())
        ))
        .execute(&conn)
        .expect("Database error");
}

// #[cached(time = 3600)] TODO: Activate
fn frequencies() -> HashMap<i64, f64> {
    let conn = crate::database::establish_connection();
    use crate::schema::achievements::dsl::*;

    let unlocked_achievements: Vec<i64> = achievements.select(achievementId)
        .load(&conn)
        .expect("Database error");

    let mut result = HashMap::new();
    let num_users = crate::schema::users::dsl::users.count()
        .get_result::<i64>(&conn)
        .expect("Database error") as f64;

    for achievement in ACHIEVEMENTS.iter() {
        let achievement_id = achievement["id"].as_i64().unwrap();
        let freq = unlocked_achievements.iter()
            .filter(|other_id| &&achievement_id == other_id)
            .count() as f64;
        result.insert(achievement_id, freq / num_users * 100.0);
    }

    result
}