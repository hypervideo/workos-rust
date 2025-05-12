use derive_more::{Deref, Display, From};
use serde::{Deserialize, Serialize};

use crate::{Timestamp, Timestamps};

use super::UserId;

/// The ID of a [`EmailVerification`].
#[derive(
    Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[from(forward)]
pub struct EmailVerificationId(String);

/// The one-time code that was emailed to the user.
#[derive(
    Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[from(forward)]
pub struct EmailVerificationCode(String);

/// [WorkOS Docs: Email verification](https://workos.com/docs/reference/user-management/email-verification)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmailVerification {
    /// The unique ID of the email verification code.
    pub id: EmailVerificationId,

    /// The unique ID of the user.
    pub user_id: UserId,

    /// The email address of the user.
    pub email: String,

    /// The timestamp indicating when the email verification code expires.
    pub expires_at: Timestamp,

    /// The one-time code that was emailed to the user.
    pub code: EmailVerificationCode,

    /// The timestamps for the user.
    #[serde(flatten)]
    pub timestamps: Timestamps,
}
