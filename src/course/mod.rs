use diesel::prelude::*;

pub mod tasks;

pub fn name_to_title(course: &str) -> Option<String> {
    use crate::schema::courses;
    courses::table.filter(courses::name.eq(&course))
        .select(courses::title)
        .first::<String>(&crate::database_connection())
        .ok()
}