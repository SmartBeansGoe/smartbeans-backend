use rocket::http::Status;
use serde_json::Value;

pub struct AchievementTrigger {
    /// All achievements as JSON values (see data/achievements.json)
    achievements: Vec<Value>,
    /// A list with the ids of open achievements
    open: Vec<i64>,
    /// All submissions as JSON values
    submissions: Vec<Value>,
}

impl AchievementTrigger {
    pub fn new() -> Result<AchievementTrigger, Status> {
        let achievements = serde_json::from_str::<Value>(
            &std::fs::read_to_string("data/achievements.json").unwrap()
        ).unwrap().as_array().unwrap().clone();

        let open = Vec::new(); // TODO
        let submissions = Vec::new(); // TODO

        Ok(AchievementTrigger {
            achievements,
            open,
            submissions
        })
    }

    pub fn run(&self, trigger: &str) {
        let trigger = Value::String(trigger.to_string());

        let ids = self.achievements.iter()
            .filter(|achievement| {
                self.open.contains(&achievement["id"].as_i64().unwrap())
            })
            .filter(|achievement| {
                trigger.as_str() == Some("all")
                    || achievement["triggers"].as_array().unwrap().contains(&trigger)
            })
            .map(|achievement| {
                achievement["id"].as_i64().unwrap()
            })
            .collect::<Vec<_>>();

        for id in ids {
            if self.check(id) {
                // TODO: Insert into table
                // TODO: Add frontend message
            }
        }
    }

    fn check(&self, id: i64) -> bool {
        match id {
            1 => self.check_1(),
            _ => panic!("Non-existent achievement id")
        }
    }

    // === Check functions for individual achievements =====

    fn check_1(&self) -> bool {
        false
    }
}

/// Returns achievements for a user.
/// Return format: [ { "id": ..., "name": ..., "description": ..., "solved" ... }, ... ]
pub fn achievements(token: &str) -> Value {
    unimplemented!()
}