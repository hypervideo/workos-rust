use std::fmt::{Display, Write};

use serde::{Serialize, Serializer, ser};

/// A [`Vec`] that can be URL-encoded.
#[derive(Debug)]
pub(crate) struct UrlEncodableVec<T: Display>(Vec<T>);

impl<T> Serialize for UrlEncodableVec<T>
where
    T: Display,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut serialized = String::new();

        for (index, item) in self.0.iter().enumerate() {
            write!(&mut serialized, "{}", item).map_err(|err| {
                ser::Error::custom(format!("failed to write '{}': {}", item, err))
            })?;

            if index < self.0.len() - 1 {
                write!(&mut serialized, ",").map_err(|err| {
                    ser::Error::custom(format!("failed to write separator: {}", err))
                })?
            }
        }

        serializer.serialize_str(&serialized)
    }
}

impl<T> From<Vec<T>> for UrlEncodableVec<T>
where
    T: Display,
{
    fn from(vec: Vec<T>) -> Self {
        Self(vec)
    }
}

#[cfg(test)]
mod test {
    use mockito::Matcher;
    use reqwest::StatusCode;
    use serde::Serialize;

    use super::*;

    #[tokio::test]
    async fn it_serializes_a_vec_in_the_query_string() {
        #[derive(Debug, Serialize)]
        struct List<'a> {
            #[serde(rename = "items[]")]
            pub items: UrlEncodableVec<&'a str>,
        }

        let mut server = mockito::Server::new_async().await;

        server
            .mock("GET", "/")
            .match_query(Matcher::UrlEncoded(
                "items[]".to_string(),
                "one,two,three".to_string(),
            ))
            .with_status(200)
            .create_async()
            .await;

        let client = reqwest::Client::new();

        let response = client
            .get(server.url())
            .query(&List {
                items: UrlEncodableVec(vec!["one", "two", "three"]),
            })
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK)
    }

    #[tokio::test]
    async fn it_serializes_a_vec_in_an_option_in_the_query_string() {
        #[derive(Debug, Serialize)]
        struct List<'a> {
            #[serde(rename = "items[]")]
            pub items: Option<UrlEncodableVec<&'a str>>,
        }

        let mut server = mockito::Server::new_async().await;

        server
            .mock("GET", "/")
            .match_query(Matcher::UrlEncoded(
                "items[]".to_string(),
                "one,two,three".to_string(),
            ))
            .with_status(200)
            .create_async()
            .await;

        let client = reqwest::Client::new();

        let response = client
            .get(server.url())
            .query(&List {
                items: Some(UrlEncodableVec(vec!["one", "two", "three"])),
            })
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK)
    }
}
