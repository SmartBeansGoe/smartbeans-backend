#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_json;

pub mod schema;
pub mod models;

pub mod routes;
pub mod guards;
pub mod smartape;
pub mod achievements;

use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};

/// Struct can be used as a request guard whenever a connection
/// to the main database is needed.
#[database("main_db")]
pub struct MainDbConn(diesel::SqliteConnection);

/// Convert rocket::Data into String (because FromData for String is only available in debug mode).
/// To be honest, I am not sure if this is more secure than implementing FromData for String, but at
/// least the compiler is happy and I will just assume noone will try to DoS our SmartBeans.
pub fn data_to_string(data: rocket::Data) -> String {
    let mut data_str = String::new();
    data.open().read_to_string(&mut data_str).unwrap();
    data_str
}

pub fn epoch() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}