use crate::config::Logo;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Group {
    name: String,
    items: Vec<Link>,
}

#[derive(Deserialize, Serialize)]
struct Link {
    name: String,
    logo: Option<Logo>,
    url: String,
    description: Option<String>,
    blank: Option<bool>,
    target: Option<String>,
}
