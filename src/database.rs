use diesel::prelude::*;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    MysqlConnection::establish(&env::var("DATABASE_URL").expect("DATABASE_URL must be set")).unwrap()
}