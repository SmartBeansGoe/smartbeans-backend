table! {
    courseMapping (studipId) {
        studipId -> Varchar,
        courseName -> Varchar,
    }
}

table! {
    courses (name) {
        name -> Varchar,
        title -> Text,
    }
}

table! {
    courseTask (course, taskid) {
        course -> Varchar,
        taskid -> Integer,
        tags -> Text,
        orderBy -> Integer,
        prerequisites -> Text,
    }
}

table! {
    sessions (token) {
        token -> Varchar,
        username -> Varchar,
        courseName -> Varchar,
        expirationTime -> Bigint,
        tokenName -> Nullable<Text>,
    }
}

table! {
    tasks (taskid) {
        taskid -> Integer,
        taskDescription -> Text,
        solution -> Text,
        lang -> Text,
        tests -> Text,
    }
}

table! {
    users (username) {
        username -> Varchar,
        displayName -> Text,
        password -> Nullable<Text>,
        ltiEnabled -> Bool,
        charData -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    courseMapping,
    courses,
    courseTask,
    sessions,
    tasks,
    users,
);
