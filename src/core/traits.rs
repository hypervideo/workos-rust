use std::fmt::Display;

use async_trait::async_trait;
use reqwest::IntoUrl;
use url::Url;

#[async_trait]
///An HTP client
pub trait Client {
    fn get(&self, url: Url) -> Box<dyn ClientRequest + '_>;
    fn post(&self, url: Url) -> Box<dyn ClientRequest + '_>;
    fn put(&self, url: Url) -> Box<dyn ClientRequest + '_>;
    fn delete(&self, url: Url) -> Box<dyn ClientRequest + '_>;
}
#[async_trait]
impl Client for reqwest::Client {
    fn get(&self, url: Url) -> Box<dyn ClientRequest + '_> {
        Box::new(self.get(url))
    }
    fn post(&self, url: Url) -> Box<dyn ClientRequest + '_> {
        Box::new(self.post(url))
    }
    fn put(&self, url: Url) -> Box<dyn ClientRequest + '_> {
        Box::new(self.put(url))
    }
    fn delete(&self, url: Url) -> Box<dyn ClientRequest + '_> {
        Box::new(self.delete(url))
    }
}
#[async_trait]
pub trait ClientRequest {
    fn bearer_auth<'a>(self: Box<Self>, x: &(dyn Display + '_)) -> Box<dyn ClientRequest + 'a>
    where
        Self: 'a;
    fn json<'a>(
        self: Box<Self>,
        x: &(dyn erased_serde::Serialize + '_),
    ) -> Box<dyn ClientRequest + 'a>
    where
        Self: 'a;
    fn query<'a>(
        self: Box<Self>,
        x: &(dyn erased_serde::Serialize + '_),
    ) -> Box<dyn ClientRequest + 'a>
    where
        Self: 'a;
    fn form<'a>(
        self: Box<Self>,
        x: &(dyn erased_serde::Serialize + '_),
    ) -> Box<dyn ClientRequest + 'a>
    where
        Self: 'a;
    async fn send(self: Box<Self>) -> Result<reqwest::Response, reqwest::Error>;
}
#[async_trait]
impl ClientRequest for reqwest::RequestBuilder {
    async fn send(self: Box<Self>) -> Result<reqwest::Response, reqwest::Error> {
        reqwest::RequestBuilder::send(*self).await
    }
    fn bearer_auth<'a>(self: Box<Self>, x: &(dyn Display + '_)) -> Box<dyn ClientRequest + 'a>
    where
        Self: 'a,
    {
        Box::new((*self).bearer_auth(x))
    }
    fn json<'a>(
        self: Box<Self>,
        x: &(dyn erased_serde::Serialize + '_),
    ) -> Box<dyn ClientRequest + 'a>
    where
        Self: 'a,
    {
        Box::new((*self).json(x))
    }
    fn query<'a>(
        self: Box<Self>,
        x: &(dyn erased_serde::Serialize + '_),
    ) -> Box<dyn ClientRequest + 'a>
    where
        Self: 'a,
    {
        Box::new((*self).query(x))
    }
    fn form<'a>(
        self: Box<Self>,
        x: &(dyn erased_serde::Serialize + '_),
    ) -> Box<dyn ClientRequest + 'a>
    where
        Self: 'a,
    {
        Box::new((*self).form(x))
    }
}
