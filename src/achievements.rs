use rocket::http::Status;
use serde_json::Value;
use diesel::prelude::*;
use diesel::insert_into;

pub struct AchievementTrigger {
    /// User name
    username: String,
    /// Token
    token: String,
    /// All achievements as JSON values (see data/achievements.json)
    achievements: Vec<Value>,
    /// A list with the ids of completed achievements
    completed: Vec<i64>,
    /// A list with all completed tasks
    solved_tasks: Vec<i64>,
    /// All submissions as JSON values
    submissions: Vec<Value>,
}

impl AchievementTrigger {
    pub fn new(user: &crate::guards::User) -> Result<AchievementTrigger, Status> {
        let submissions = crate::smartape::all_submissions(&user.token)?
            .as_array().unwrap().to_owned();

        Ok(AchievementTrigger {
            username: String::from(&user.name),
            token: String::from(&user.token),
            achievements: achievements_from_datafile(),
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
                achievement_completed(&self.username, id);
                // TODO: Add frontend message
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
        self.solved_tasks.len() >= crate::smartape::tasks(&self.token).unwrap().as_array().unwrap().len()
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
/// Return format: [ { "id": ..., "name": ..., "description": ..., "completed" ... }, ... ]
pub fn achievements(username: &str) -> Value {
    let mut result = Vec::new();

    let completed_vec = completed_achievements(username);
    for achievement in achievements_from_datafile() {
        let mut val = json!({});

        let id = achievement["id"].as_i64().unwrap();
        let completed = completed_vec.contains(&id);

        val["id"] = achievement["id"].clone();
        val["name"] = achievement["name"].clone();
        val["description"] = if completed {
            achievement["description"]["completed"].clone()
        } else {
            achievement["description"]["open"].clone()
        };
        val["completed"] = serde_json::to_value(completed).unwrap();

        if achievement["hidden"].as_bool() == Some(false) || completed {
            result.push(val);
        }
    }

    serde_json::to_value(result).unwrap()
}

pub fn completed_achievements(name: &str) -> Vec<i64> {
    use crate::schema::achievements::dsl::*;
    let conn = crate::database::establish_connection();

    achievements.filter(username.eq(name))
        .select(achievementId)
        .load(&conn)
        .expect("Database error")
}

fn achievements_from_datafile() -> Vec<Value> {
    serde_json::from_str::<Value>(
        &std::fs::read_to_string("data/achievements.json").unwrap()
    ).unwrap().as_array().unwrap().clone()
}

fn achievement_completed(uname: &str, achievemet_id: i64) {
    use crate::schema::achievements::dsl::*;
    let conn = crate::database::establish_connection();

    insert_into(achievements).values((
            username.eq(uname),
            achievementId.eq(achievemet_id),
            completionTime.eq(crate::epoch())
        ))
        .execute(&conn)
        .expect("Database error");
}