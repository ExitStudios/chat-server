use crate::models::{message::Message, user::User};

pub struct ServerState {
    pub messages: Vec<Message>,
    pub users: Vec<User>,
    pub next_message_id: u32,
}


