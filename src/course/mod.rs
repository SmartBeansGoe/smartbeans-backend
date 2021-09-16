use diesel::prelude::*;

pub mod tasks;

pub fn exists(course: &str) -> bool {
    use crate::schema::courses;
    courses::table.filter(courses::name.eq(&course))
        .first::<(String, String)>(&crate::database_connection())
        .is_ok()
}