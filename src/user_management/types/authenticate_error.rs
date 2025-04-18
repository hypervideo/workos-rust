use async_trait::async_trait;
use reqwest::{Response, StatusCode};
use serde::Deserialize;
use thiserror::Error;

use crate::{WorkOsError, WorkOsResult};

/// An error returned from authenticate requests.
#[derive(Debug, Error, Deserialize)]
#[error("{error}: {error_description}")]
pub struct AuthenticateError {
    /// The error code of the error that occurred.
    pub error: String,

    /// The description of the error.
    pub error_description: String,
}

#[async_trait]
pub(crate) trait HandleAuthenticateError
where
    Self: Sized,
{
    async fn handle_authenticate_error(self) -> WorkOsResult<Self, AuthenticateError>;
}

#[async_trait]
impl HandleAuthenticateError for Response {
    async fn handle_authenticate_error(self) -> WorkOsResult<Self, AuthenticateError> {
        match self.error_for_status_ref() {
            Ok(_) => Ok(self),
            Err(err) => match err.status() {
                Some(StatusCode::BAD_REQUEST) => {
                    let error = self.json::<AuthenticateError>().await?;

                    Err(match error.error.as_str() {
                        "invalid_client" | "unauthorized_client" => WorkOsError::Unauthorized,
                        _ => WorkOsError::Operation(error),
                    })
                }
                Some(StatusCode::FORBIDDEN) => {
                    let error = self.json::<AuthenticateError>().await?;

                    Err(WorkOsError::Operation(error))
                }
                _ => Err(WorkOsError::RequestError(err)),
            },
        }
    }
}
