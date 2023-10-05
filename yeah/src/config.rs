use serde::{Deserialize, Serialize};

use crate::group::Group;
use crate::message::Message;
use crate::search::{Search, SearchConfig};
use crate::statistic::StatisticConfig;

#[derive(Deserialize, Serialize)]
struct AppConfig {
    // 标题
    title: Option<String>,
    // 副标题
    subtitle: Option<String>,
    // 标签页标题
    tab_title: Option<String>,
    // 网站图标地址
    favicon: Option<String>,
    // 主题色
    color: Option<String>,
    // 功能配置
    features: Option<FeaturesConfig>,
    // 消息配置
    messages: Option<Vec<Message>>,
    // 分组配置
    groups: Option<Vec<Group>>,
}

#[derive(Deserialize, Serialize)]
struct FeaturesConfig {
    // 默认搜索引擎
    search: Option<Search>,
    // 自定义搜索引擎
    custom_search: Option<Vec<SearchConfig>>,
    // 便签
    note: Option<NoteConfig>,
    // 网址提交
    submit: Option<SubmitConfig>,
    // 网址反馈
    feedback: Option<FeedbackConfig>,
    // 网址统计
    statistics: Option<Vec<StatisticConfig>>,
    // 网址备案
    record: Option<RecordConfig>,
    // 网址监控
    monitor: Option<MonitorConfig>,
    // 网址压缩
    compression: Option<CompressionConfig>,
}

type NoteConfig = bool;

type SubmitConfig = bool;

type FeedbackConfig = bool;

#[derive(Deserialize, Serialize)]
struct RecordConfig {
    // 备案号
    number: String,
    // 备案链接
    url: String,
}

type MonitorConfig = bool;

type CompressionConfig = bool;

#[derive(Deserialize, Serialize)]
pub enum Logo {
    Emj(String),
    Img(String),
}