use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum Message {
    Primary(String),
    Success(String),
    Warning(String),
    Danger(String),
}
