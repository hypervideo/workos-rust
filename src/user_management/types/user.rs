use std::fmt::Display;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::{Metadata, Timestamp, Timestamps};

/// The ID of a [`User`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct UserId(String);

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for UserId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for UserId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

/// [WorkOS Docs: User](https://workos.com/docs/reference/user-management/user)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// The unique ID of the user.
    pub id: UserId,

    /// The email address of the user.
    pub email: String,

    /// The first name of the user.
    pub first_name: Option<String>,

    /// The last name of the user.
    pub last_name: Option<String>,

    /// Whether the user's email has been verified.
    pub email_verified: bool,

    /// A URL reference to an image representing the user.
    pub profile_picture_url: Option<Url>,

    /// The timestamp when the user last signed in.
    pub last_sign_in_at: Option<Timestamp>,

    /// The external ID of the user.
    pub external_id: Option<String>,

    /// Object containing metadata key/value pairs associated with the user.
    pub metadata: Metadata,

    /// The timestamps for the user.
    #[serde(flatten)]
    pub timestamps: Timestamps,
}
