#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate dotenv_codegen;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_json;

pub mod schema;
pub mod models;

pub mod routes;
pub mod guards;

/// Struct can be used as a request guard whenever a connection
/// to the main database is needed.
#[database("main_db")]
pub struct MainDbConn(diesel::SqliteConnection);