use derive_more::{Deref, Display, From};
use serde::Serialize;

/// An API key to authenticate with the WorkOS API.
#[derive(Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[from(forward)]
pub struct ApiKey(String);
