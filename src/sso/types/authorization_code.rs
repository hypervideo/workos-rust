use std::fmt::Display;

use serde::Serialize;

/// An authorization code that may be exchanged for an SSO profile and access
/// token.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct AuthorizationCode(String);

impl Display for AuthorizationCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for AuthorizationCode {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for AuthorizationCode {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
