use crate::objects::*;
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
    // pub boost_num: i32,
    // pub level: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelShort {
    pub id: String,
    pub name: String,
    pub master_id: String,
    pub parent_id: String,
    #[serde(rename = "type")]
    pub ty: u8,
    pub level: i64,
    pub limit_amount: i64,
    pub is_category: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelView {
    pub id: String,
    pub guild_id: String,
    pub master_id: String,
    pub parent_id: String,
    pub name: String,
    pub topic: String,
    #[serde(rename = "type")]
    pub ty: u8,
    pub level: i64,
    pub slow_mode: i64,
    pub limit_amount: i64,
    pub is_category: bool,
    pub server_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emoji {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RespList<T> {
    pub items: Vec<T>,
    pub meta: PageMeta,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GuildUserList {
    pub items: Vec<User>,
    pub meta: PageMeta,
    pub user_count: i32,
    pub online_count: i32,
    pub offline_count: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MessageResp {
    pub msg_id: String,
    pub msg_timestamp: i64,
    pub nonce: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GuildMuteList {
    pub mic: GuildMuteItem,
    pub headset: GuildMuteItem,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GuildMuteItem {
    #[serde(rename = "type")]
    pub ty: u8,
    pub user_ids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Gateway {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DirectMessage {
    pub id: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub content: String,
    // pub embeds: Vec<>, todo
    pub attachments: Vec<Attachments>,
    pub create_at: u64,
    pub updated_at: u64,
    // pub reactions: Vec<>, todo
    pub author_id: String,
    pub image_name: String,
    pub read_status: bool,
    // pub quote: Option<String>, todo
    // pub mention_info: Option<String>, todo
}

#[derive(Debug, Clone, Deserialize)]
pub struct MuteList {
    pub mic: MuteItem,
    pub headset: MuteItem,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MuteItem {
    #[serde(rename = "type")]
    pub ty: u8,
    pub user_ids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChannelRole {
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub permission_users: Vec<PermissionUser>,
    pub permission_sync: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfUser {
    pub id: String,
    pub username: String,
    pub nickname: String,
    pub identify_num: String,
    pub online: bool,
    pub bot: bool,
    pub status: u8,
    pub avatar: String,
    // pub mobile_verify: bool, bug!
    pub mobile_prefix: String,
    pub mobile: String,
    pub invited_count: String, //bug!
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetUrl {
    pub url: String,
}
