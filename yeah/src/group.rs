use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::config::AppConfig;
use crate::service::Service;

#[derive(
Deserialize,
Serialize,
Clone, // Sometimes clone is needed :(
)]
pub struct Group {
    // 分组名称
    name: String,
    // 分组项目
    items: Vec<Service>,
}

pub async fn get_groups(State(config): State<Arc<AppConfig>>) -> Json<Vec<Group>> {
    Json(config.groups.clone().unwrap())
}
