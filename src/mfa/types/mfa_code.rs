use derive_more::{Deref, Display, From};
use serde::Serialize;

/// A multi-factor authentication (MFA) code.
#[derive(Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[from(forward)]
pub struct MfaCode(String);
