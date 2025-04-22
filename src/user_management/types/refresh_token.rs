use derive_more::{Deref, Display, From};
use serde::{Deserialize, Serialize};

/// A refresh token that may be exchanged for a new [`AccessToken`](crate::sso::AccessToken).
#[derive(
    Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[from(forward)]
pub struct RefreshToken(String);
