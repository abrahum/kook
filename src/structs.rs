use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageMeta {
    pub page: i32,
    pub page_total: i32,
    pub page_size: i32,
    pub total: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildShort {
    pub id: String,
    pub name: String,
    pub topic: String,
    pub master_id: String,
    pub icon: String,
    pub notify_type: u8,
    pub region: String,
    pub enable_open: bool,
    pub open_id: String,
    pub default_channel_id: String,
    pub welcome_channel_id: String,
    pub boost_nem: i32,
    pub level: i32,
}
