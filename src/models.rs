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

#[derive(Serialize, Queryable)]
pub struct Character {
    pub username: String,
    pub body_color: Option<String>,
    pub hat_id: Option<String>,
    pub face_id: Option<String>,
    pub shirt_id: Option<String>,
    pub pants_id: Option<String>
}

#[derive(Queryable)]
pub struct SystemMessage {
    pub id: i32,
    pub user: String,
    pub message_type: String,
    pub time: i64,
    pub content: String
}