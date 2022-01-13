use crate::prelude::*;
use hyper::{
    body::{aggregate, Buf},
    client::{Client, HttpConnector},
    header::{AUTHORIZATION, CONTENT_TYPE},
    Body, Request, Uri,
};
use hyper_tls::HttpsConnector;

const V3_BASE_URL: &str = "https://www.kaiheila.cn/api/v3";
pub(crate) type HttpsClient = Client<HttpsConnector<HttpConnector>>;

impl<const V: u8> crate::KHL<V> {
    pub(crate) fn new_https_clent() -> HttpsClient {
        let https = HttpsConnector::new();
        Client::builder().build::<_, Body>(https)
    }

    fn build_url(paths: Vec<&str>, query: &str) -> Uri {
        let mut url = format!("{}/{}", V3_BASE_URL, paths.join("/"));
        if !query.is_empty() {
            url.push_str("?");
            url.push_str(query);
        }
        url.parse().unwrap()
    }

    async fn _get(&self, url: Uri) -> KHLResult<impl Buf> {
        self.limit.check_limit(url.path()).await;
        let req = Request::get(url)
            .header(AUTHORIZATION, &self.author)
            .body(Body::empty())
            .unwrap();
        let res = self.http_client.request(req).await?;
        self.limit.update_from_header(res.headers()).await;
        aggregate(res).await.map_err(|e| e.into())
    }

    async fn _post(&self, url: Uri, body: String) -> KHLResult<impl Buf> {
        self.limit.check_limit(url.path()).await;
        let req = Request::post(url)
            .header(AUTHORIZATION, &self.author)
            .header(CONTENT_TYPE, "application/json")
            .body(body.into())
            .unwrap();
        let res = self.http_client.request(req).await?;
        self.limit.update_from_header(res.headers()).await;
        aggregate(res).await.map_err(|e| e.into())
    }

    async fn get<T>(&self, url: Uri) -> KHLResult<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let body = self._get(url).await?;
        let data: HttpResp<EmptyAble<T>> = serde_json::from_reader(body.reader())?;
        data.as_result()
    }

    async fn post<T>(&self, url: Uri, body: String) -> KHLResult<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let body = self._post(url, body).await?;
        let data: HttpResp<EmptyAble<T>> = serde_json::from_reader(body.reader())?;
        data.as_result()
    }

    // async fn empty_get(&self, url: Uri) -> KHLResult<()> {
    //     let body = self._get(url).await?;
    //     let data: HttpResp<Empty> = serde_json::from_str(&body)?;
    //     data.as_result()
    // }

    async fn empty_post(&self, url: Uri, body: String) -> KHLResult<()> {
        let body = self._post(url, body).await?;
        let data: HttpResp<Empty> = serde_json::from_reader(body.reader())?;
        data.as_result()
    }

    pub async fn get_guild_list(
        &self,
        page: Option<i32>,
        page_size: Option<i32>,
        sort: Option<&str>,
    ) -> KHLResult<RespList<GuildShort>> {
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
    ) -> KHLResult<()> {
        let mut query = Map::default();
        query
            .push("guild_id", guild_id)
            .push_option("user_id", user_id)
            .push_option("nickname", nickname);
        let url = Self::build_url(vec!["guild", "nickname"], "");
        self.empty_post(url, query.build()).await
    }

    pub async fn leave_guild(&self, guild_id: &str) -> KHLResult<()> {
        let url = Self::build_url(vec!["guild", "leave"], "");
        let mut query = Map::default();
        query.push("guild_id", guild_id);
        self.empty_post(url, query.build()).await
    }

    pub async fn kickout_guild(&self, guild_id: &str, target_id: &str) -> KHLResult<()> {
        let url = Self::build_url(vec!["guild", "kickout"], "");
        let mut query = Map::default();
        query
            .push("guild_id", guild_id)
            .push("target_id", target_id);
        self.empty_post(url, query.build()).await
    }

    pub async fn get_guild_mute_list(
        &self,
        guild_id: &str,
        return_type: Option<&str>,
    ) -> KHLResult<GuildMuteList> {
        let mut query = QueryBuilder::new("guild_id", guild_id);
        query.push_option("return_type", return_type);
        let url = Self::build_url(vec!["guild-mute", "list"], &query.build());
        self.get(url).await
    }

    pub async fn set_guild_mute(&self, guild_id: &str, user_id: &str, ty: u8) -> KHLResult<()> {
        let url = Self::build_url(vec!["guild-mute", "create"], "");
        let mut query = Map::default();
        query
            .push("guild_id", guild_id)
            .push("user_id", user_id)
            .push("type", ty);
        self.empty_post(url, query.build()).await
    }

    pub async fn unset_guild_mute(&self, guild_id: &str, user_id: &str) -> KHLResult<()> {
        let url = Self::build_url(vec!["guild-mute", "delete"], "");
        let mut query = Map::default();
        query.push("guild_id", guild_id).push("user_id", user_id);
        self.empty_post(url, query.build()).await
    }

    pub async fn get_channel_list(
        &self,
        guild_id: &str,
        page: Option<i32>,
        page_size: Option<i32>,
        ty: Option<i32>,
    ) -> KHLResult<RespList<ChannelShort>> {
        let mut query = QueryBuilder::new("guild_id", guild_id);
        query
            .push_option("page", page)
            .push_option("page_size", page_size)
            .push_option("type", ty);
        let url = Self::build_url(vec!["channel", "list"], &query.build());
        self.get(url).await
    }

    pub async fn get_channel_view(&self, channel_id: &str) -> KHLResult<Channel> {
        //?
        let url = Self::build_url(
            vec!["channel", "view"],
            &QueryBuilder::new("channel_id", channel_id).build(),
        );
        self.get(url).await
    }

    pub async fn send_channel_message(
        &self,
        target_id: &str,
        content: &str,
        ty: Option<u8>,
        quote: Option<&str>,
        nonce: Option<&str>,
        temp_target_id: Option<&str>,
    ) -> KHLResult<MessageResp> {
        let url = Self::build_url(vec!["channel", "message"], "");
        let mut query = Map::default();
        query
            .push("target_id", target_id)
            .push("content", content)
            .push_option("type", ty)
            .push_option("quote", quote)
            .push_option("nonce", nonce)
            .push_option("temp_target_id", temp_target_id);
        self.post(url, query.build()).await
    }

    pub async fn send_direct_message(
        &self,
        target_id: Option<&str>,
        chat_code: Option<&str>,
        content: String,
        ty: Option<i32>,
        quote: Option<&str>,
        nonce: Option<&str>,
    ) -> KHLResult<MessageResp> {
        let mut query = Map::default();
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
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MapValue {
    Int(i32),
    String(String),
}

impl From<u8> for MapValue {
    fn from(value: u8) -> Self {
        MapValue::Int(value as i32)
    }
}

impl From<i32> for MapValue {
    fn from(value: i32) -> Self {
        MapValue::Int(value)
    }
}

impl From<&str> for MapValue {
    fn from(value: &str) -> Self {
        MapValue::String(value.to_string())
    }
}

impl From<String> for MapValue {
    fn from(value: String) -> Self {
        MapValue::String(value)
    }
}

type Map = HashMap<String, MapValue>;

trait MapExt {
    fn push<T>(&mut self, key: &str, value: T) -> &mut Self
    where
        T: Into<MapValue>;
    fn push_option<T>(&mut self, key: &str, value: Option<T>) -> &mut Self
    where
        T: Into<MapValue>;
    fn build(&self) -> String;
}

impl MapExt for Map {
    fn push<T>(&mut self, key: &str, value: T) -> &mut Self
    where
        T: Into<MapValue>,
    {
        self.insert(key.to_string(), value.into());
        self
    }
    fn push_option<T>(&mut self, key: &str, value: Option<T>) -> &mut Self
    where
        T: Into<MapValue>,
    {
        if let Some(value) = value {
            self.insert(key.to_string(), value.into());
        }
        self
    }
    fn build(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResp<T> {
    pub code: i32,
    pub message: String,
    pub data: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmptyAble<T> {
    Data(T),
    Empty {},
}

impl<T> HttpResp<EmptyAble<T>> {
    pub fn as_result(self) -> KHLResult<T> {
        if self.code == 0 {
            match self.data {
                EmptyAble::Empty {} => Err(KHLError::HttpApiCallEmptyResponse),
                EmptyAble::Data(data) => Ok(data),
            }
        } else {
            Err(KHLError::HttpApiCallError(self.message))
        }
    }
}

impl HttpResp<Empty> {
    pub fn as_result(self) -> KHLResult<()> {
        if self.code == 0 {
            Ok(())
        } else {
            Err(KHLError::HttpApiCallError(self.message))
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Empty {}

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

#[tokio::test]
async fn test() {
    let config = Config::load_from_file();
    let khl = crate::KHL::new_from_config(config, EchoHandler).arc();
    let list = khl.get_guild_list(None, None, None).await.unwrap();
    println!("{:?}", list);
    // let guild = khl.get_guild_view(&list.items[0].id).await.unwrap();
    // println!("{:?}", guild);
    // let msg = khl
    //     .send_direct_message(
    //         None,
    //         Some("3575610837"),
    //         None,
    //         "Hello World".to_string(),
    //         None,
    //         None,
    //     )
    //     .await;
    // println!("{:?}", msg);
}
