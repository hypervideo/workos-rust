use derive_more::{Deref, Display, From};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{Metadata, Timestamp, Timestamps};

/// The ID of a [`User`].
#[derive(
    Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[from(forward)]
pub struct UserId(String);

/// [WorkOS Docs: User](https://workos.com/docs/reference/user-management/user)
#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub metadata: Option<Metadata>,

    /// The timestamps for the user.
    #[serde(flatten)]
    pub timestamps: Timestamps,
}
