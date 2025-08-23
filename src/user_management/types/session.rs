use std::net::IpAddr;

use derive_more::{Deref, Display, From};
use serde::{Deserialize, Serialize};

use crate::{
    KnownOrUnknown, Timestamp, Timestamps, organizations::OrganizationId, user_management::UserId,
};

/// The ID of a [`Session`].
#[derive(
    Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[from(forward)]
pub struct SessionId(String);

/// The state of an [`Session`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    /// The session is active.
    Active,

    /// The session is expired.
    Expired,

    /// The session is revoked.
    Revoked,
}

/// The state of an [`Session`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionAuthMethod {
    /// The session was authenticated using extenal authentication.
    ExternalAuth,

    /// The session was authenticated using impersenation.
    Impersenation,

    /// The session was authenticated using a magic code.
    MagicCode,

    /// The session was authenticated using a migrated session.
    MigratedSession,

    /// The session was authenticated using OAuth.
    Oauth,

    /// The session was authenticated using passkey.
    Passkey,

    /// The session was authenticated using password.
    Password,

    /// The session was authenticated using SSO.
    SSO,

    /// The session was authenticated using an unknown method.
    Unknown,
}

/// [WorkOS Docs: Session](https://workos.com/docs/reference/user-management/session)
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    /// The unique ID of the session.
    pub id: SessionId,

    /// The user ID of the session.
    pub user_id: UserId,

    /// The organization ID of the session.
    pub organization_id: Option<OrganizationId>,

    /// The status of the session.
    pub status: KnownOrUnknown<SessionStatus, String>,

    /// The authentication method of the session.
    pub auth_method: KnownOrUnknown<SessionAuthMethod, String>,

    /// The IP address of the session.
    pub ip_address: Option<IpAddr>,

    /// The user agent of the session.
    pub user_agent: Option<String>,

    /// The timestamp indicating when the session expires.
    pub expires_at: Timestamp,

    /// The timestamp indicating when the session was ended.
    pub ended_at: Option<Timestamp>,

    /// The timestamps for the session.
    #[serde(flatten)]
    pub timestamps: Timestamps,
}
