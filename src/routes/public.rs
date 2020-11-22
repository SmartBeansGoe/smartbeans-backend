#[get("/version")]
pub fn version() -> String {
    env!("GIT_HASH").to_string()
}