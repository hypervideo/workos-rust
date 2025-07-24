use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;

use crate::{WorkOsError, WorkOsResult, traits::ClientResponse};

pub trait ResponseExt<'a>
where
    Self: Sized,
{
    /// Handles an unauthorized error from the WorkOS API by converting it into a
    /// [`WorkOsError::Unauthorized`] response.
    fn handle_unauthorized_error<E>(self) -> WorkOsResult<Box<dyn ClientResponse + 'a>, E>;

    /// Handles a generic error from the WorkOS API by converting it into a
    /// [`WorkOsError::RequestError`] response.
    fn handle_generic_error<E>(self) -> WorkOsResult<Box<dyn ClientResponse + 'a>, E>;

    /// Handles an unauthorized or generic error from the WorkOS API.
    fn handle_unauthorized_or_generic_error<E>(
        self,
    ) -> WorkOsResult<Box<dyn ClientResponse + 'a>, E>;

    async fn json<T: DeserializeOwned, E>(self) -> WorkOsResult<T, E>;
}

impl<'a> ResponseExt<'a> for Box<dyn ClientResponse + 'a> {
    fn handle_unauthorized_error<E>(self) -> WorkOsResult<Box<dyn ClientResponse + 'a>, E> {
        if self.status() == StatusCode::UNAUTHORIZED {
            Err(WorkOsError::Unauthorized)
        } else {
            Ok(self)
        }
    }

    fn handle_generic_error<E>(self) -> WorkOsResult<Box<dyn ClientResponse + 'a>, E> {
        match self.error_for_status() {
            Ok(response) => Ok(response),
            Err(err) => Err(WorkOsError::RequestError(err)),
        }
    }

    fn handle_unauthorized_or_generic_error<E>(
        self,
    ) -> WorkOsResult<Box<dyn ClientResponse + 'a>, E> {
        self.handle_unauthorized_error()?.handle_generic_error()
    }

    async fn json<T: DeserializeOwned, E>(self) -> WorkOsResult<T, E> {
        let t = self.text().await.map_err(WorkOsError::RequestError)?;

        Ok(serde_json::from_str(&t)?)
    }
}
