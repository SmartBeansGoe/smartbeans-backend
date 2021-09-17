#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_json;

use config::Config;
use diesel::prelude::*;

#[allow(non_snake_case)]
pub mod schema;

pub mod auth;
pub mod user;
pub mod course;
pub mod tools;

lazy_static! {
    pub static ref SETTINGS: Config = {
        let mut config = Config::default();
        // Merge default configuration file
        config.merge(config::File::with_name("SettingsDefault.toml"))
            .expect("Missing default config");
        // Merge user configuration file; ignore if it doesn't exist
        config.merge(config::File::with_name("Settings.toml")).ok();
        config
    };
}

pub fn database_connection() -> MysqlConnection {
    let host: String = SETTINGS.get("database.host")
        .expect("Missing database host in settings file");
    let port: u32 = SETTINGS.get("database.port")
        .expect("Missing database port in settings file");
    let user: String = SETTINGS.get("database.user")
        .expect("Missing database user in settings file");
    let password: String = SETTINGS.get("database.password")
        .expect("Missing database password in settings file");
    let database: String = SETTINGS.get("database.database")
        .expect("Missing database name in settings file");

    MysqlConnection::establish(&format!("mysql://{}:{}@{}:{}/{}", user, password, host, port, database))
        .expect("Failed to open database connection")
}

#[get("/logged_in")]
pub fn logged_in(user: auth::guards::User) -> String {
    format!("Logged in as <b>{}</b> in course <b>{}</b>", user.name, user.course)
}