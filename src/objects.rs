use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub nickname: String,
    pub identify_num: String,
    pub online: bool,
    pub bot: bool,
    pub status: u8,
    pub avatar: String,
    pub vip_avatar: String,
    // pub mobile_verify: bool,
    pub roles: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guild {
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
    pub roles: Vec<Role>,
    pub channels: Vec<Channel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub role_id: i32,
    pub name: String,
    pub color: i32,
    pub position: i32,
    pub hoist: i32,
    pub mentionable: i32,
    pub permissions: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub user_id: String,
    pub guild_id: String,
    pub topic: String,
    #[serde(deserialize_with = "de_bool_or_int")]
    pub is_category: bool,
    pub parent_id: String,
    pub level: i32,
    pub slow_mode: i32,
    #[serde(rename = "type")]
    pub ty: i32,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub permission_users: Vec<PermissionUser>,
    pub permission_sync: i32,
    pub has_password: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionOverwrite {
    pub role_id: i32,
    pub allow: i32,
    pub deny: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionUser {
    pub user: User,
    pub allow: i32,
    pub deny: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub id: String,
    #[serde(rename = "type")]
    pub ty: i32,
    pub content: String,
    pub create_at: i32,
    pub author: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachments {
    #[serde(rename = "type")]
    pub ty: String,
    pub url: String,
    pub name: String,
    pub size: i32,
}

use serde::de::{Deserializer, Visitor};

struct BoolOrIntVisitor;

impl<'de> Visitor<'de> for BoolOrIntVisitor {
    type Value = bool;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("bool or int")
    }
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }
}

fn de_bool_or_int<'de, D>(d: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    d.deserialize_any(BoolOrIntVisitor)
}
