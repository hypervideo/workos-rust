use derive_more::{Deref, Display, From};
use serde::{Deserialize, Serialize};

/// An access token that may be exchanged for a [`Profile`](crate::sso::Profile).
#[derive(
    Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[from(forward)]
pub struct AccessToken(String);
