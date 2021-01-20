use serde_json::Value;

use std::collections::HashMap;

lazy_static! {
    pub static ref ACHIEVEMENTS: Vec<Value> = serde_json::from_str(
        include_str!("../data/achievements.json")
    ).unwrap();

    pub static ref ASSETS: Vec<Value> = serde_json::from_str(
        include_str!("../data/assets.json")
    ).unwrap();

    pub static ref TASK_STATS: HashMap<i64, Value> = {
        serde_json::from_str::<Vec<Value>>(include_str!("../data/task_stats.json")).unwrap()
            .into_iter()
            .fold(HashMap::new(), |mut map, task| {
                map.insert(task["taskid"].as_i64().unwrap(), task);
                map
            })
    };
}