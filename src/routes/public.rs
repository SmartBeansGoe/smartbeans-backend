#[get("/version")]
pub fn version() -> String {
    format!("{} ({})", env!("CARGO_PKG_VERSION"), env!("GIT_HASH"))
}