use std::collections::HashMap;

use crate::prelude::*;
#[cfg(not(test))]
use hyper::body::{aggregate, Buf};
use hyper::{
    client::{Client, HttpConnector},
    header::{AUTHORIZATION, CONTENT_TYPE},
    Body, Request, Uri,
};
use hyper_tls::HttpsConnector;
use tracing::trace;

const V3_BASE_URL: &str = "https://www.kaiheila.cn/api/v3";
pub(crate) type HttpsClient = Client<HttpsConnector<HttpConnector>>;

impl crate::Kook {
    pub(crate) fn new_https_client() -> HttpsClient {
        let https = HttpsConnector::new();
        Client::builder().build::<_, Body>(https)
    }

    pub async fn get<T>(&self, url: Vec<&str>, query: QueryBuilder) -> KHLResult<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let bucket = url.join("/");
        let mut url = format!("{}/{}", V3_BASE_URL, bucket);
        let query = query.build_query();
        if !query.is_empty() {
            url.push('?');
            url.push_str(&query);
        }
        let url: Uri = url.parse().unwrap();
        trace!(target: KOOK, "Calling api GET {}", url);
        self.limit.check_limit(&bucket).await;
        let req = Request::get(url)
            .header(AUTHORIZATION, &self.author)
            .body(Body::empty())
            .unwrap();
        let res = self.http_client.request(req).await?;
        self.limit.update_from_header(res.headers(), &bucket).await;
        #[cfg(test)]
        {
            use hyper::body::to_bytes;
            let bytes = to_bytes(res.into_body())
                .await
                .map_err(|e| KHLError::HyperError(e))?;
            let s = String::from_utf8(bytes.to_vec()).unwrap();
            trace!(target: crate::KOOK, "post resp: {:?}", s);
            let data: HttpResp<EmptyAble<T>> = serde_json::from_str(&s)?;
            data.as_result()
        }
        #[cfg(not(test))]
        {
            let body = aggregate(res).await.map_err(|e| KHLError::HyperError(e))?;
            let data: HttpResp<EmptyAble<T>> = serde_json::from_reader(body.reader())?;
            data.as_result()
        }
    }

    pub async fn post<T>(&self, url: Vec<&str>, query: QueryBuilder) -> KHLResult<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let bucket = url.join("/");
        let url: Uri = format!("{}/{}", V3_BASE_URL, bucket).parse().unwrap();
        self.limit.check_limit(&bucket).await;
        let data = query.json();
        trace!(target: KOOK, "Calling api POST {} {}", bucket, data);
        let req = Request::post(url)
            .header(AUTHORIZATION, &self.author)
            .header(CONTENT_TYPE, "application/json")
            .body(data.into())
            .unwrap();
        let res = self.http_client.request(req).await?;
        self.limit.update_from_header(res.headers(), &bucket).await;
        #[cfg(test)]
        {
            use hyper::body::to_bytes;
            let bytes = to_bytes(res.into_body())
                .await
                .map_err(|e| KHLError::HyperError(e))?;
            let s = String::from_utf8(bytes.to_vec()).unwrap();
            tracing::trace!(target: crate::KOOK, "get resp: {:?}", s);
            let data: HttpResp<EmptyAble<T>> = serde_json::from_str(&s)?;
            data.as_result()
        }
        #[cfg(not(test))]
        {
            let body = aggregate(res).await?;
            let data: HttpResp<EmptyAble<T>> = serde_json::from_reader(body.reader())?;
            data.as_result()
        }
    }

    pub async fn empty_post(&self, url: Vec<&str>, query: QueryBuilder) -> KHLResult<()> {
        self.post::<JsonValue>(url, query).await?;
        Ok(())
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
