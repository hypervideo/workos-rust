use reqwest::{Response, StatusCode};

use crate::{WorkOsError, WorkOsResult};

pub trait ResponseExt
where
    Self: Sized,
{
    /// Handles an unauthorized error from the WorkOS API by converting it into a
    /// [`WorkOsError::Unauthorized`] response.
    async fn handle_unauthorized_error<E>(self) -> WorkOsResult<Self, E>;

    /// Handles a generic error from the WorkOS API by converting it into a
    /// [`WorkOsError::RequestError`] response.
    async fn handle_generic_error<E>(self) -> WorkOsResult<Self, E>;

    /// Handles an unauthorized or generic error from the WorkOS API.
    async fn handle_unauthorized_or_generic_error<E>(self) -> WorkOsResult<Self, E>;
}

impl ResponseExt for Response {
    async fn handle_unauthorized_error<E>(self) -> WorkOsResult<Self, E> {
        if self.status() == StatusCode::UNAUTHORIZED {
            Err(WorkOsError::Unauthorized)
        } else {
            Ok(self)
        }
    }

    async fn handle_generic_error<E>(self) -> WorkOsResult<Self, E> {
        if self.status().is_success() {
            Ok(self)
        } else {
            let status = self.status();
            let body = self.text().await.unwrap_or_else(|_| "Failed to read response body".to_string());
            Err(WorkOsError::ApiError { status, body })
        }
    }

    async fn handle_unauthorized_or_generic_error<E>(self) -> WorkOsResult<Self, E> {
        self.handle_unauthorized_error().await?.handle_generic_error().await
    }
}
