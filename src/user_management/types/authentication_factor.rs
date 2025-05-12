use derive_more::{Deref, Display, From};
use serde::{Deserialize, Serialize};

use crate::Timestamps;

use super::UserId;

/// The ID of an [`AuthenticationFactor`].
#[derive(
    Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[from(forward)]
pub struct AuthenticationFactorId(String);

/// The type of the authentication factor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthenticationFactorTypeString {
    /// Time-based one-time password (TOTP).
    Totp,
}

/// The type of the authentication factor.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AuthenticationFactorType {
    /// Time-based one-time password (TOTP).
    Totp {
        /// Your application or company name displayed in the user’s authenticator app. Defaults to your WorkOS team name.
        issuer: String,

        /// The user’s account name displayed in their authenticator app. Defaults to the user’s email.
        user: String,

        /// Base64 encoded image containing scannable QR code.
        qr_code: String,

        /// TOTP secret that can be manually entered into some authenticator apps in place of scanning a QR code.
        secret: String,

        /// The `otpauth` URI that is encoded by the provided `qr_code`.
        uri: String,
    },
}

/// The ID and name of an [`AuthenticationFactor`].
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct AuthenticationFactorIdAndType {
    /// The unique ID of the authentication factor.
    pub id: AuthenticationFactorId,

    /// The type of the authentication factor.
    pub r#type: AuthenticationFactorTypeString,
}

/// [WorkOS Docs: AuthenticationFactor](https://workos.com/docs/reference/organization)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthenticationFactor {
    /// The unique ID of the authentication factor.
    pub id: AuthenticationFactorId,

    /// The type of the factor to enroll.
    pub r#type: AuthenticationFactorType,

    ///The ID of the user.
    pub user_id: UserId,

    /// The timestamps for the authentication factor.
    #[serde(flatten)]
    pub timestamps: Timestamps,
}
