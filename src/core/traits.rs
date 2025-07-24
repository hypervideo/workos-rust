use std::fmt::Display;

use async_trait::async_trait;
use http::StatusCode;
use url::Url;

pub trait StatusError: std::error::Error {
    fn status(&self) -> Option<StatusCode>;
}

impl StatusError for reqwest::Error {
    fn status(&self) -> Option<StatusCode> {
        self.status()
    }
}

#[derive(Debug)]
pub struct RequestError {
    pub err: Box<dyn StatusError + Send + Sync>,
}

impl StatusError for RequestError {
    fn status(&self) -> Option<StatusCode> {
        self.err.status()
    }
}

impl RequestError {
    pub fn status(&self) -> Option<StatusCode> {
        StatusError::status(self)
    }
}

impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.err)
    }
}
impl std::error::Error for RequestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.err)
    }
}

impl From<reqwest::Error> for RequestError {
    fn from(value: reqwest::Error) -> Self {
        Self {
            err: Box::new(value),
        }
    }
}

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
pub trait ClientResponse: Send + Sync {
    fn status(&self) -> StatusCode;
    fn error_for_status<'a>(self: Box<Self>) -> Result<Box<dyn ClientResponse + 'a>, RequestError>
    where
        Self: 'a;
    fn error_for_status_ref(&self) -> Result<&(dyn ClientResponse + '_), RequestError>;
    async fn text(self: Box<Self>) -> Result<String, RequestError>;
}
#[async_trait]
impl ClientResponse for reqwest::Response {
    fn status(&self) -> StatusCode {
        self.status()
    }
    fn error_for_status<'a>(self: Box<Self>) -> Result<Box<dyn ClientResponse + 'a>, RequestError>
    where
        Self: 'a,
    {
        match (*self).error_for_status() {
            Err(e) => Err(e.into()),
            Ok(a) => Ok(Box::new(a)),
        }
    }
    fn error_for_status_ref(&self) -> Result<&(dyn ClientResponse + '_), RequestError> {
        match self.error_for_status_ref() {
            Err(e) => Err(e.into()),
            Ok(v) => Ok(v),
        }
    }
    async fn text(self: Box<Self>) -> Result<String, RequestError> {
        (*self).text().await.map_err(Into::into)
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
    async fn send<'a>(self: Box<Self>) -> Result<Box<dyn ClientResponse + 'a>, RequestError>
    where
        Self: 'a;
}
#[async_trait]
impl ClientRequest for reqwest::RequestBuilder {
    async fn send<'a>(self: Box<Self>) -> Result<Box<dyn ClientResponse + 'a>, RequestError>
    where
        Self: 'a,
    {
        match reqwest::RequestBuilder::send(*self).await {
            Err(e) => Err(e.into()),
            Ok(a) => Ok(Box::new(a)),
        }
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
