use derive_more::{Deref, Display, From};
use serde::{Deserialize, Serialize};

/// The authentication token returned from a failed authentication attempt due to the corresponding error.
#[derive(
    Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[from(forward)]
pub struct PendingAuthenticationToken(String);
