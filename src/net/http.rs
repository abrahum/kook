use crate::prelude::*;
use reqwest::{Client, Url};

const V3_BASE_URL: &str = "https://www.kaiheila.cn/api/v3";
pub(crate) type HttpClient = Client;

impl crate::KHL<3> {
    fn build_url(paths: Vec<&str>, query: &str) -> Url {
        let mut url = format!("{}/{}", V3_BASE_URL, paths.join("/"));
        if !query.is_empty() {
            url.push_str("?");
            url.push_str(query);
        }
        Url::parse(&url).unwrap()
    }

    async fn get<T>(&self, url: Url) -> KHLResult<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let res = self.http_client.get(url).send().await?;
        let body = res.text().await?;
        println!("{}", body);
        let data: HttpResp<T> = serde_json::from_str(&body)?;
        data.as_result()
    }

    async fn post<T>(&self, url: Url, data: String) -> KHLResult<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        println!("{}", data);
        let res = self.http_client.post(url).body(data).send().await?;
        let body = res.text().await?;
        println!("{}", body);
        let data: HttpResp<T> = serde_json::from_str(&body)?;
        data.as_result()
    }

    pub async fn get_guild_list(
        &self,
        page: Option<i32>,
        page_size: Option<i32>,
        sort: Option<&str>,
    ) -> KHLResult<GuildList> {
        let mut query = QueryBuilder::default();
        query
            .push_option("page", page)
            .push_option("page_size", page_size)
            .push_option("sort", sort);
        let url = Self::build_url(vec!["guild", "list"], &query.build());
        self.get(url).await
    }

    pub async fn get_guild_view(&self, guild_id: &str) -> KHLResult<Guild> {
        let url = Self::build_url(
            vec!["guild", "view"],
            &QueryBuilder::new("guild_id", guild_id).build(),
        );
        self.get(url).await
    }

    pub async fn get_guild_user_list(
        &self,
        guild_id: &str,
        channel_id: Option<&str>,
        search: Option<&str>,
        role_id: Option<i32>,
        mobile_verified: bool,
        active_time: bool,
        joined_at: bool,
        page: Option<i32>,
        page_size: Option<i32>,
        filter_user_id: Option<&str>,
    ) -> KHLResult<GuildUserList> {
        let mut query = QueryBuilder::new("guild_id", guild_id);
        query
            .push_option("channel_id", channel_id)
            .push_option("search", search)
            .push_option("role_id", role_id)
            .push_bool("mobile_verified", mobile_verified)
            .push_bool("active_time", active_time)
            .push_bool("joined_at", joined_at)
            .push_option("page", page)
            .push_option("page_size", page_size)
            .push_option("filter_user_id", filter_user_id);
        let url = Self::build_url(vec!["guild", "user-list"], &query.build());
        self.get(url).await
    }

    pub async fn set_guild_user_nickname(
        &self,
        guild_id: &str,
        user_id: Option<&str>,
        nickname: Option<&str>,
    ) -> KHLResult<Empty> {
        let mut query = QueryBuilder::new("guild_id", guild_id);
        query
            .push_option("user_id", user_id)
            .push_option("nickname", nickname);
        let url = Self::build_url(vec!["guild", "nickname"], "");
        self.post(url, query.build()).await
    }

    pub async fn leave_guild(&self, guild_id: &str) -> KHLResult<Empty> {
        let url = Self::build_url(vec!["guild", "leave"], "");
        self.post(url, QueryBuilder::new("guild_id", guild_id).build())
            .await
    }

    pub async fn kickout_guild(&self, guild_id: &str, target_id: &str) -> KHLResult<Empty> {
        let url = Self::build_url(vec!["guild", "kickout"], "");
        let mut query = QueryBuilder::new("guild_id", guild_id);
        query.push("target_id", target_id);
        self.post(url, query.build()).await
    }

    pub async fn set_guild_mute(&self, guild_id: &str, user_id: &str, ty: u8) -> KHLResult<Empty> {
        let url = Self::build_url(vec!["guild-mute", "create"], "");
        let mut query = QueryBuilder::new("guild_id", guild_id);
        query.push("user_id", user_id).push("type", ty);
        self.post(url, query.build()).await
    }

    pub async fn send_direct_message(
        &self,
        ty: Option<i32>,
        target_id: Option<&str>,
        chat_code: Option<&str>,
        content: String,
        quote: Option<&str>,
        nonce: Option<&str>,
    ) -> KHLResult<MessageResp> {
        let mut query = QueryBuilder::default();
        query
            .push_option("type", ty)
            .push_option("target_id", target_id)
            .push_option("chat_code", chat_code)
            .push("content", content)
            .push_option("quote", quote)
            .push_option("nonce", nonce);
        let url = Self::build_url(vec!["direct-message", "create"], "");
        self.post(url, query.build()).await
    }

    pub async fn get_gateway(&self, compress: bool) -> KHLResult<Gateway> {
        let mut query = QueryBuilder::default();
        query.push_bool("compress", compress);
        let url = Self::build_url(vec!["gateway", "index"], &query.build());
        self.get(url).await
    }
}

#[derive(Debug, Default)]
struct QueryBuilder {
    params: Vec<String>,
}

impl QueryBuilder {
    pub fn new<T>(key: &str, value: T) -> QueryBuilder
    where
        T: ToString,
    {
        let mut builder = QueryBuilder::default();
        builder.push(key, value);
        builder
    }

    pub fn push<T>(&mut self, key: &str, value: T) -> &mut Self
    where
        T: ToString,
    {
        self.params.push(format!("{}={}", key, value.to_string()));
        self
    }

    pub fn push_option<T>(&mut self, key: &str, value: Option<T>) -> &mut Self
    where
        T: ToString,
    {
        if let Some(value) = value {
            self.push(key, value);
        }
        self
    }

    pub fn push_bool(&mut self, key: &str, value: bool) -> &mut Self {
        if value {
            self.push(key, 1);
        } else {
            self.push(key, 0);
        }
        self
    }

    pub fn build(self) -> String {
        self.params.join("&")
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResp<T> {
    pub code: i32,
    pub message: String,
    pub data: T,
}

impl<T> HttpResp<T> {
    pub fn as_result(self) -> KHLResult<T> {
        if self.code == 0 {
            Ok(self.data)
        } else {
            Err(KHLError::HttpApiCallError(self.message))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildList {
    pub items: Vec<GuildShort>,
    pub meta: PageMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildUserList {
    pub items: Vec<User>,
    pub meta: PageMeta,
    pub user_count: i32,
    pub online_count: i32,
    pub offline_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Empty {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResp {
    pub msg_id: String,
    pub msg_timestamp: i64,
    pub nonce: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gateway {
    pub url: String,
}

#[tokio::test]
async fn test() {
    let config = Config::load_from_file();
    let khl = crate::KHL::new_from_config(config).arc();
    // let list = khl.get_guild_list(None, None, None).await.unwrap();
    // println!("{:?}", list);
    // let guild = khl.get_guild_view(&list.items[0].id).await.unwrap();
    // println!("{:?}", guild);
    let msg = khl
        .send_direct_message(
            None,
            Some("3575610837"),
            None,
            "Hello World".to_string(),
            None,
            None,
        )
        .await;
    println!("{:?}", msg);
}
