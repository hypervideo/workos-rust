use thiserror::Error;

/// A WorkOS SDK error.
#[derive(Debug, Error)]
pub enum WorkOsError<E> {
    /// An error occurred with the current operation.
    #[error("operational error")]
    Operation(E),

    /// An unauthorized response was received from the WorkOS API.
    #[error("unauthorized")]
    Unauthorized,

    /// An error occurred while parsing a URL.
    #[error("URL parse error")]
    UrlParseError(#[from] url::ParseError),

    /// An error occurred while parsing an IP address.
    #[error("IP addres parse error")]
    IpAddrParseError(#[from] std::net::AddrParseError),

    /// An unhandled error occurred with the API request.
    #[error("request error")]
    RequestError(#[from] reqwest::Error),
}

/// A WorkOS SDK result.
pub type WorkOsResult<T, E> = Result<T, WorkOsError<E>>;
