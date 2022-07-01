use std::collections::HashMap;

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

impl crate::Kook {
    pub(crate) fn new_https_clent() -> HttpsClient {
        let https = HttpsConnector::new();
        Client::builder().build::<_, Body>(https)
    }

    async fn _post(&self, url: Vec<&str>, query: QueryBuilder) -> KHLResult<impl Buf> {
        let url: Uri = format!("{}/{}", V3_BASE_URL, url.join("/"))
            .parse()
            .unwrap();
        self.limit.check_limit(url.path()).await;
        let req = Request::post(url)
            .header(AUTHORIZATION, &self.author)
            .header(CONTENT_TYPE, "application/json")
            .body(query.json().into())
            .unwrap();
        let res = self.http_client.request(req).await?;
        self.limit.update_from_header(res.headers()).await;
        aggregate(res).await.map_err(|e| e.into())
    }

    pub async fn get<T>(&self, url: Vec<&str>, query: QueryBuilder) -> KHLResult<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let mut url = format!("{}/{}", V3_BASE_URL, url.join("/"));
        let query = query.build_query();
        if !query.is_empty() {
            url.push('?');
            url.push_str(&query);
        }
        let url: Uri = url.parse().unwrap();
        self.limit.check_limit(url.path()).await;
        let req = Request::get(url)
            .header(AUTHORIZATION, &self.author)
            .body(Body::empty())
            .unwrap();
        let res = self.http_client.request(req).await?;
        self.limit.update_from_header(res.headers()).await;
        let body = aggregate(res).await.map_err(|e| KHLError::HyperError(e))?;
        let data: HttpResp<EmptyAble<T>> = serde_json::from_reader(body.reader())?;
        data.as_result()
    }

    pub async fn post<T>(&self, url: Vec<&str>, query: QueryBuilder) -> KHLResult<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let body = self._post(url, query).await?;
        let data: HttpResp<EmptyAble<T>> = serde_json::from_reader(body.reader())?;
        data.as_result()
    }

    pub async fn empty_post(&self, url: Vec<&str>, query: QueryBuilder) -> KHLResult<()> {
        let body = self._post(url, query).await?;
        let data: HttpResp<JsonValue> = serde_json::from_reader(body.reader())?;
        data.as_result()
    }
}

#[derive(Debug, Default)]
pub struct QueryBuilder {
    params: HashMap<&'static str, JsonValue>,
}

impl QueryBuilder {
    pub fn push<T: Into<JsonValue>>(&mut self, key: &'static str, value: T) {
        self.params.insert(key, value.into());
    }

    pub fn build_query(self) -> String {
        self.params
            .into_iter()
            .filter_map(|(k, v)| match v {
                JsonValue::Null => None,
                JsonValue::Bool(b) => Some(format!("{}={}", k, if b { 1 } else { 0 })),
                JsonValue::String(s) => Some(format!("{}={}", k, s)),
                v => Some(format!("{}={}", k, v)),
            })
            .collect::<Vec<String>>()
            .join("&")
    }

    pub fn json(self) -> String {
        serde_json::to_string(&self.params).unwrap()
    }
}

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

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

impl HttpResp<JsonValue> {
    pub fn as_result(self) -> KHLResult<()> {
        if self.code == 0 {
            Ok(())
        } else {
            Err(KHLError::HttpApiCallError(self.message))
        }
    }
}
