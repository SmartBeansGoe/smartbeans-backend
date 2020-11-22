#[get("/version")]
pub fn version() -> String {
    "Hello World".to_string()
}