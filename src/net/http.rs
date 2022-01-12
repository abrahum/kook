use crate::prelude::*;
use reqwest::{Client, Url};

const V3_BASE_URL: &str = "https://www.kaiheila.cn/api/v3";
pub(crate) type HttpClient = Client;

impl crate::KHL<3> {
    fn get_url(paths: Vec<&str>) -> Url {
        let mut url = Url::parse(V3_BASE_URL).unwrap();
        url.path_segments_mut().unwrap().push(&paths.join("/"));
        url
    }

    async fn get<T>(&self, url: Url) -> KHLResult<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let res = self.http_client.get(url).send().await?;
        let body = res.text().await?;
        let data: HttpResp<T> = serde_json::from_str(&body)?;
        Ok(data.data)
    }

    async fn post<T>(&self, url: Url, data: String) -> KHLResult<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let res = self.http_client.post(url).body(data).send().await?;
        let body = res.text().await?;
        let data: HttpResp<T> = serde_json::from_str(&body)?;
        Ok(data.data)
    }

    pub async fn get_guild_list(
        &self,
        page: Option<i32>,
        page_size: Option<i32>,
        sort: Option<&str>,
    ) -> KHLResult<GuildList> {
        let mut url = Self::get_url(vec!["guilds", "list"]);
        url.push_option_query("page", page)
            .push_option_query("page_size", page_size)
            .push_option_query("sort", sort);
        Ok(self.get(url).await?)
    }

    pub async fn leave_guild(&self, guild_id: &str) -> KHLResult<()> {
        let url = Self::get_url(vec!["guilds", "leave"]);
        let _: Empty = self
            .post(url, build_params(vec![("guild_id", guild_id)]))
            .await?;
        Ok(())
    }

    pub async fn set_guild_mute(&self, guild_id: &str, user_id: &str, ty: u8) -> KHLResult<()> {
        let url = Self::get_url(vec!["guilds-mute", "create"]);
        let _: Empty = self
            .post(
                url,
                build_params(vec![
                    ("guild_id", guild_id),
                    ("user_id", user_id),
                    ("type", &ty.to_string()),
                ]),
            )
            .await?;
        Ok(())
    }

    pub async fn get_gateway(&self, compress: bool) -> KHLResult<Gateway> {
        let mut url = Self::get_url(vec!["gateway", "index"]);
        if !compress {
            url.push_query("compress", 0);
        }
        Ok(self.get(url).await?)
    }
}

fn build_params(params: Vec<(&str, &str)>) -> String {
    let v = params
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>();
    v.join("&")
}

trait AddQuery {
    fn push_option_query<T>(&mut self, key: &str, query: Option<T>) -> &mut Self
    where
        T: ToString;
    fn push_query<T>(&mut self, key: &str, query: T) -> &mut Self
    where
        T: ToString;
}

impl AddQuery for Url {
    fn push_option_query<T>(&mut self, key: &str, query: Option<T>) -> &mut Self
    where
        T: ToString,
    {
        if let Some(query) = query {
            self.query_pairs_mut().append_pair(key, &query.to_string());
        }
        self
    }
    fn push_query<T>(&mut self, key: &str, query: T) -> &mut Self
    where
        T: ToString,
    {
        self.query_pairs_mut().append_pair(key, &query.to_string());
        self
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResp<T> {
    pub code: i32,
    pub message: String,
    pub data: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildList {
    pub items: Vec<GuildShort>,
    pub meta: PageMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Empty {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gateway {
    pub url: String,
}
