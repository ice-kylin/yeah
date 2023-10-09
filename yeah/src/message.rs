use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::config::AppConfig;

#[derive(Deserialize, Serialize, Clone)]
pub struct Message {
    msg_type: MessageType,
    title: String,
    content: String,
}

#[derive(Deserialize, Serialize, Clone)]
enum MessageType {
    Primary,
    Success,
    Warning,
    Danger,
}

pub async fn get_message(State(config): State<Arc<AppConfig>>) -> Json<Vec<Message>> {
    Json(config.messages.clone().unwrap())
}
