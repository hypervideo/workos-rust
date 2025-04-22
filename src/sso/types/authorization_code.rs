use derive_more::{Deref, Display, From};
use serde::Serialize;

/// An authorization code that may be exchanged for an SSO profile and access
/// token.
#[derive(Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[from(forward)]
pub struct AuthorizationCode(String);
