use std::net::IpAddr;

use serde::{Deserialize, Serialize};

use crate::{KnownOrUnknown, user_management::UserId};

/// The type of a [`AuthenticationEvent`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthenticationEventType {
    /// The authentication event is related to SSO.
    Sso,

    /// The authentication event is related to password.
    Password,

    /// The authentication event is related to OAuth.
    Oauth,

    /// The authentication event is related to MFA.
    Mfa,

    /// The authentication event is related to magic auth.
    MagicAuth,

    /// The authentication event is related to email verification.
    EmailVerification,
}

/// The status of a [`AuthenticationEvent`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthenticationEventStatus {
    /// The authentication event failed.
    Failed,

    /// The authentication event succeeded.
    Succeeded,
}

/// The error of a [`AuthenticationEvent`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticationEventError {
    /// The error code.
    pub code: String,

    /// The error message.
    pub message: String,
}

/// [WorkOS Docs: Authentication events](https://workos.com/docs/events/authentication)
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticationEvent {
    /// The type of the authentication event.
    pub r#type: KnownOrUnknown<AuthenticationEventType, String>,

    /// The status of the authentication event.
    pub status: KnownOrUnknown<AuthenticationEventStatus, String>,

    /// The user ID of the authentication event.
    pub user_id: Option<UserId>,

    /// The email of the authentication event.
    pub email: Option<String>,

    /// The IP address of the authentication event.
    pub ip_address: Option<IpAddr>,

    /// The user agent of the authentication event.
    pub user_agent: Option<String>,

    /// The error of the authentication event.
    pub error: Option<AuthenticationEventError>,
}
