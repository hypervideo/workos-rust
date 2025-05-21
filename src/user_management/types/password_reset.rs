use derive_more::{Deref, Display, From};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::Timestamp;

use super::UserId;

/// The ID of a [`PasswordReset`].
#[derive(
    Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[from(forward)]
pub struct PasswordResetId(String);

/// The one-time token that can be used to reset a user's password.
#[derive(
    Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[from(forward)]
pub struct PasswordResetToken(String);

/// [WorkOS Docs: Password Reset](https://workos.com/docs/reference/user-management/password-reset)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PasswordReset {
    /// The unique ID of the password reset token.
    pub id: PasswordResetId,

    /// The unique ID of the user.
    pub user_id: UserId,

    /// The email address of the user.
    pub email: String,

    /// The one-time token that can be used to reset a user's password.
    pub password_reset_token: PasswordResetToken,

    /// The URL that can be used to reset a user's password.
    pub password_reset_url: Url,

    /// The timestamp indicating when the password reset token expires.
    pub expires_at: Timestamp,

    /// The timestamp indicating when the object was created.
    pub created_at: Timestamp,
}
