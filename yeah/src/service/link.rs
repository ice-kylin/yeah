use serde::{Deserialize, Serialize};

use crate::common::{Href, Logo};

#[derive(Deserialize, Serialize, Clone)]
pub struct Link {
    // 网址名称
    name: String,
    // 网址图标
    logo: Option<Logo>,
    // 网址链接
    href: Href,
    // 不匹配所有规则时是否隐藏
    hide: Option<HideConfig>,
    // 网址描述
    description: Option<String>,
    // 网址是否在新窗口打开
    blank: Option<bool>,
    // 自定义 target 属性
    target: Option<String>,
}

type HideConfig = bool;
