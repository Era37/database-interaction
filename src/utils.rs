use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ChatBotLog {
    pub user_query: String,
    pub computer_response: String,
}
