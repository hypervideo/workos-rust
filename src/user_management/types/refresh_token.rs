use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// A refresh token that may be exchanged for a new [`AccessToken`](crate::sso::AccessToken).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RefreshToken(String);

impl Display for RefreshToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for RefreshToken {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for RefreshToken {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
