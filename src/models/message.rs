use serde::{Deserialize, Serialize};

use crate::models::user::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub user: User,
    pub text: String,
}
