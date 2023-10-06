use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::config::{AppConfig, Logo};

#[derive(Deserialize, Serialize, Clone)]
pub struct Group {
    name: String,
    items: Vec<Link>,
}

#[derive(Deserialize, Serialize, Clone)]
struct Link {
    name: String,
    logo: Option<Logo>,
    url: String,
    description: Option<String>,
    blank: Option<bool>,
    target: Option<String>,
}

pub async fn groups_handler(State(config): State<Arc<AppConfig>>) -> Json<Vec<Group>> {
    Json(config.groups.clone().unwrap())
}
