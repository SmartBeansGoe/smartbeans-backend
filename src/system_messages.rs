use diesel::prelude::*;
use crate::models::SystemMessage;

pub fn send_message(username: &str, message_type: &str, message_content: &str) {
    use crate::schema::system_messages::dsl::*;

    diesel::insert_into(system_messages)
        .values((
            user.eq(username),
            messageType.eq(message_type),
            content.eq(message_content),
            time.eq(crate::epoch())
        ))
        .execute(&crate::database::establish_connection())
        .expect("Database error");
}

pub fn receive_messages(username: &str) -> Vec<SystemMessage> {
    use crate::schema::system_messages::dsl::*;
    let conn = crate::database::establish_connection();

    let messages = system_messages.filter(user.eq(username))
        .load(&conn)
        .expect("Database error");

    diesel::delete(system_messages.filter(user.eq(username)))
        .execute(&conn)
        .expect("Database error");

    messages
}