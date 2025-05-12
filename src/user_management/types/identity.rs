use derive_more::{Deref, Display, From};
use serde::{Deserialize, Serialize};

use super::OauthProvider;

/// The ID of a [`Identity`].
#[derive(
    Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[from(forward)]
pub struct IdentityId(String);

/// The type of the identity.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum IdentityType {
    /// OAuth identity.
    OAuth {
        /// The type of OAuth provider for the identity.
        provider: OauthProvider,
    },
}

/// [WorkOS Docs: Identity](https://workos.com/docs/reference/user-management/identity)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Identity {
    /// The unique ID of the user in the external identity provider.
    pub idp_id: IdentityId,

    /// The type of the identity.
    #[serde(flatten)]
    pub r#type: IdentityType,
}
