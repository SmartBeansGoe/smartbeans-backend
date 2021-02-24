#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate lazy_static;

pub mod schema;
pub mod models;

pub mod database;
pub mod routes;
pub mod guards;
pub mod smartape;
pub mod achievements;
pub mod init_user;
pub mod system_messages;
pub mod static_data;
pub mod level;
pub mod bot;

use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};
use diesel::prelude::*;

/// Convert rocket::Data into String (because FromData for String is only available in debug mode).
/// To be honest, I am not sure if this is more secure than implementing FromData for String, but at
/// least the compiler is happy and I will just assume noone will try to DoS our SmartBeans.
pub fn data_to_string(data: rocket::Data) -> String {
    let mut data_str = String::new();
    data.open().read_to_string(&mut data_str).unwrap();

    if data_str.len() >= 30000 {
        data_str[..30000].to_string()
    }
    else {
        data_str
    }
}

/// Returns the current time in seconds since 1970-01-01.
pub fn epoch() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

pub fn log_route(username_param: &str, route_param: &str, data_param: Option<&str>) {
    use crate::schema::route_log::dsl::*;

    diesel::insert_into(route_log)
        .values((
            username.eq(username_param),
            time.eq(epoch()),
            route.eq(route_param),
            data.eq(data_param)
        ))
        .execute(&database::establish_connection())
        .expect("Database error");

}