use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub enum Logo {
    Emj(String),
    Img(String),
}

#[derive(Deserialize, Serialize, Clone)]
pub enum Href {
    Url(String),
    Rules(Rules),
}

type Rules = HashMap<String, String>;
