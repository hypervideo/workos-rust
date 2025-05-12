use derive_more::{Deref, Display, From};
use serde::{Deserialize, Serialize};

use crate::{Timestamp, Timestamps};

use super::UserId;

/// The ID of a [`MagicAuth`].
#[derive(
    Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[from(forward)]
pub struct MagicAuthId(String);

/// The one-time code that was emailed to the user.
#[derive(
    Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[from(forward)]
pub struct MagicAuthCode(String);

/// [WorkOS Docs: Magic Auth](https://workos.com/docs/reference/user-management/magic-auth)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MagicAuth {
    /// The unique ID of the Magic Auth code.
    pub id: MagicAuthId,

    /// The unique ID of the user.
    pub user_id: UserId,

    /// The email address of the user.
    pub email: String,

    /// The timestamp indicating when the Magic Auth code expires.
    pub expires_at: Timestamp,

    /// The one-time code that was emailed to the user.
    pub code: MagicAuthCode,

    /// The timestamps for the user.
    #[serde(flatten)]
    pub timestamps: Timestamps,
}
