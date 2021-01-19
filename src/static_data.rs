use serde_json::Value;

lazy_static! {
    pub static ref ACHIEVEMENTS: Vec<Value> = serde_json::from_str(
        include_str!("../data/achievements.json")
    ).unwrap();

    pub static ref ASSETS: Vec<Value> = serde_json::from_str(
        include_str!("../data/assets.json")
    ).unwrap();
}