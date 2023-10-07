use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::config::{AppConfig, Logo};

#[derive(
    Deserialize,
    Serialize,
    Clone, // Sometimes clone is needed :(
)]
pub struct Group {
    // 分组名称
    name: String,
    // 分组项目
    items: Vec<Link>,
}

#[derive(
    Deserialize,
    Serialize,
    Clone, // Sometimes clone is needed :(
)]
struct Link {
    // 网址名称
    name: String,
    // 网址图标
    logo: Option<Logo>,
    // 网址链接
    url: String,
    // 网址描述
    description: Option<String>,
    // 网址是否在新窗口打开
    blank: Option<bool>,
    // 自定义 target 属性
    target: Option<String>,
}

pub async fn get_groups(State(config): State<Arc<AppConfig>>) -> Json<Vec<Group>> {
    Json(config.groups.clone().unwrap())
}
