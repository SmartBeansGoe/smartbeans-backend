use rocket::http::Status;
use serde_json::{Value, Number};
use diesel::prelude::*;
use diesel::insert_into;
//use cached::proc_macro::cached;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::{HashMap, HashSet};

use crate::static_data::ACHIEVEMENTS;
use std::f64::consts::PI;

lazy_static! {
    static ref QUEUE: Mutex<HashMap<String, (bool, HashSet<String>)>> = Mutex::new(HashMap::new());
}

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
    /// User's points
    points: HashMap<String, u64>
}

impl AchievementTrigger {
    pub fn run(user: &crate::guards::User, trigger: &str) -> Result<(), Status> {
        if !AchievementTrigger::lock(&user.name, trigger) {
            return Ok(());
        }

        AchievementTrigger {
            username: String::from(&user.name),
            token: String::from(&user.token),
            achievements: &ACHIEVEMENTS,
            completed: completed_achievements(&user.name),
            solved_tasks: crate::smartape::progress(&user.token)?,
            submissions: crate::smartape::all_submissions(&user.token)?,
            points: crate::level::user_points(&user)?
        }.run_trigger(trigger.to_string());

        Ok(())
    }

    fn run_trigger(self, trigger: String) {
        std::thread::spawn(move || {
            let trigger = Value::String(trigger);

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

            for id in &ids {
                if self.check(*id) {
                    set_achievement_completed(&self.username, *id);
                }
            }

            // Special case: Achievement for unlocking all other achievements
            if ids.contains(&16) && completed_achievements(&self.username).len() == self.achievements.len() - 1 {
                set_achievement_completed(&self.username, 16);
            }

            if let Some(next) = AchievementTrigger::free(&self.username) {
                AchievementTrigger::run(
                    &crate::guards::User { name: self.username, token: self.token },
                    &next
                ).unwrap();
            }
        });
    }

    // Returns true if the check can proceed, false otherwise
    fn lock(user: &str, trigger: &str) -> bool {
        let mut queue = QUEUE.lock().unwrap();

        // If there is no queue for the user, create one
        if !queue.contains_key(user) {
            queue.insert(user.to_string(), (true, HashSet::new()));
        }

        let (free, waiting) = queue.get_mut(user).unwrap();

        // If there is no other check running, allow this one
        if *free {
            *free = false;
            return true;
        }

        // Otherwise insert it into the waiting queue
        waiting.insert(trigger.to_string());

        false
    }

    // Returns None if there is no other check waiting, a check trigger otherwise
    fn free(user: &str) -> Option<String> {
        let mut queue = QUEUE.lock().unwrap();
        let (free, waiting) = queue.get_mut(user).unwrap();

        // The current check has finished
        *free = true;

        // If there is no check waiting, return None
        if waiting.len() == 0 {
            return None;
        }

        // Otherwise take a trigger from the queue and return it
        let next = waiting.iter().next().unwrap().to_string();
        waiting.remove(&next);
        Some(next)
    }

    fn check(&self, id: i64) -> bool {
        match id {
            1 => self.check_1(),
            2 => self.check_2(),
            3 => self.check_3(),
            4 => self.check_4(),
            5 => self.check_5(),
            6 => self.check_6(),
            7 => self.check_7(),
            8 => self.check_8(),
            9 => self.check_9(),
            10 => self.check_10(),
            11 => self.check_11(),
            12 => self.check_12(),
            13 => self.check_13(),
            14 => self.check_14(),
            15 => self.check_15(),
            16 => self.check_16(),
            17 => self.check_17(),
            18 => self.check_18(),
            19 => self.check_19(),
            _ => panic!("Non-existent achievement id")
        }
    }

    // === Check functions for individual achievements =====

    // Ein erster Schritt...; login, submission
    fn check_1(&self) -> bool {
        !self.solved_tasks.is_empty()
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

    // Nur Schall und Rauch?; nickname_changed
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

    // Trollmathematik; login, submission
    fn check_7(&self) -> bool {
        self.solved_tasks.len() >= 3 && self.solved_tasks.contains(&48)
    }

    // Kompilations-Komplikationen; login, submission
    fn check_8(&self) -> bool {
        self.submissions.iter()
            .filter(|submission| {
                submission["result"]["type"] == Value::String("COMPILE_ERROR".to_string())
            })
            .count() >= 3
    }

    // Aber es muss funktionieren!; login, submission
    fn check_9(&self) -> bool {
        let mut wrong_submissions = self.submissions.iter()
            .filter(|submission| {
                submission["result"]["type"] != Value::String("SUCCESS".to_string())
            })
            .collect::<Vec<_>>();

        if wrong_submissions.is_empty() {
            return false;
        }

        wrong_submissions.sort_unstable_by(|a, b| {
            Value::as_str(&a["sourceCode"]).unwrap()
                .cmp(&Value::as_str(&b["sourceCode"]).unwrap())
        });

        for i in 0..wrong_submissions.len() - 1 {
            if wrong_submissions[i]["sourceCode"] == wrong_submissions[i+1]["sourceCode"]
                && wrong_submissions[i]["taskid"] == wrong_submissions[i+1]["taskid"] {
                return true;
            }
        }

        false
    }

    // Doppelt hält besser; login, submission
    fn check_10(&self) -> bool {
        let mut correct_submissions = self.submissions.iter()
            .filter(|submission| {
                submission["result"]["type"] == Value::String("SUCCESS".to_string())
            })
            .collect::<Vec<_>>();

        if correct_submissions.is_empty() {
            return false;
        }

        correct_submissions.sort_unstable_by(|a, b| {
            Value::as_i64(&a["taskid"]).unwrap()
                .cmp(&Value::as_i64(&b["taskid"]).unwrap())
        });

        for i in 0..correct_submissions.len() - 1 {
            if correct_submissions[i]["taskid"] == correct_submissions[i+1]["taskid"] {
                return true;
            }
        }

        false
    }

    // Multitalent; login, submission
    fn check_11(&self) -> bool {
        let total_points = crate::level::total_points();

        for (key, value) in self.points.iter() {
            if *value as f64 / (total_points[key] as f64) < 0.5 {
                return false;
            }
        }

        true
    }

    // Maximiert; login, submission
    fn check_12(&self) -> bool {
        let total_points = crate::level::total_points();
        let mut count = 0;

        for (key, value) in self.points.iter() {
            if value == &total_points[key] {
                count += 1;
            }
        }

        count >= 3
    }

    // Unentschlossen; login, char_changed
    fn check_13(&self) -> bool {
        use crate::schema::users::dsl::*;
        let counter: i64 = users.filter(username.eq(&self.username))
            .select(char_changed)
            .first(&crate::database::establish_connection())
            .expect("Database error");

        counter >= 7
    }

    // Von allem etwas; login, submission
    fn check_14(&self) -> bool {
        for value in self.points.values() {
            if value <= &0 {
                return false;
            }
        }

        true
    }

    // 404; 404
    fn check_15(&self) -> bool {
        true
    }

    // Durchgespielt; see run function
    fn check_16(&self) -> bool {
        false
    }

    // Schnörkellos; login, submission
    fn check_17(&self) -> bool {
        self.submissions.iter()
            .filter(|submission| {
                submission["taskid"] == Value::Number(Number::from(63))
                && submission["result"]["type"] == Value::String("SUCCESS".to_string())
            })
            .any(|submission| {
                !AchievementTrigger::check_for_loops(
                    submission["sourceCode"].as_str().unwrap().to_string()
                )
            })
    }

    // Returns true if the code contains a loop
    fn check_for_loops(mut code: String) -> bool {
        use regex::Regex;

        // Remove strings
        code = Regex::new(r#""(\\.|[^"])*""#).unwrap()
            .replace_all(&code, "").to_string();

        // Remove block comments
        code = Regex::new(r#"/\*(\\.|[^(\*/)])*\*/"#).unwrap()
            .replace_all(&code, "").to_string();

        // Remove line comments
        code = Regex::new(r#"//(\\.|[^(//)])*(\n|$)"#).unwrap()
            .replace_all(&code, "").to_string();

        // Search for loops
        Regex::new(r#"\b(for|while)\b"#).unwrap().is_match(&code)
    }

    // Auf dem Weg nach oben; login, submission
    fn check_18(&self) -> bool {
        crate::level::points_to_level(self.points["total"]) >= 5
    }

    // Etwas von allem; login, submission
    fn check_19(&self) -> bool {
        let mut total_points = crate::level::total_points().into_iter()
            .filter(|(skill, _)| skill != "total")
            .map(|(skill, points)| (skill, points as f64))
            .collect::<Vec<_>>();
        total_points.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        let mut user_points = self.points.iter()
            .filter(|(skill, _)| skill != &"total")
            .map(|(skill, points)| (skill, *points as f64))
            .collect::<Vec<_>>();
        user_points.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        let mut user_sum = 0.0;
        let mut total_sum = 0.0;

        let sin = (360.0 / total_points.len() as f64 * PI  / 180.0).sin();

        for i in 0..total_points.len() {
            user_sum += 0.5 * user_points[i].1 * user_points[(i+1)%user_points.len()].1 * sin;
            total_sum += 0.5 * total_points[i].1 * total_points[(i+1)%total_points.len()].1 * sin;
        }

        user_sum / total_sum >= 0.5
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

    let achievement_json = crate::achievements::achievements(uname)
        .into_iter()
        .find(|a| a["id"] == achievement_id)
        .unwrap();

    crate::system_messages::send_message(
        uname,
        "achievement_unlocked",
        &achievement_json.to_string()
    )
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