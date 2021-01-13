use diesel::prelude::*;

/// Struct can be used as a request guard whenever a connection
/// to the main database is needed.
#[database("main_db")]
pub struct MainDbConn(diesel::SqliteConnection);

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("db.sqlite").unwrap()
}