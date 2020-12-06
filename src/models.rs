use crate::schema::sessions;

#[derive(Queryable)]
pub struct Session {
    pub auth_token: String,
    pub expiration_time: i64,
    pub username: String,
    pub smartape_token: String,
}

#[derive(Insertable)]
#[table_name = "sessions"]
pub struct NewSession {
    pub auth_token: String,
    pub expiration_time: i64,
    pub username: String,
    pub smartape_token: String,
}