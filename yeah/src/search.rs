use serde::{Deserialize, Serialize};

use crate::config::Logo;

#[derive(Deserialize, Serialize)]
pub enum Search {
    Google,
    Bing,
    DuckDuckGo,
    Yahoo,
    Ecosia,
    Baidu,
    Sogou,
    Zhihu,
    Weibo,
    Github,
    Custom(SearchConfig),
}

#[derive(Deserialize, Serialize)]
pub struct SearchConfig {
    name: String,
    url: String,
    logo: Logo,
    shortcut: Option<String>,
}
