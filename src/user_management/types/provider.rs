use derive_more::Display;
use serde::{Deserialize, Serialize};

/// The type of OAuth provider.
#[derive(Clone, Copy, Debug, Display, Serialize, Deserialize)]
pub enum OauthProvider {
    /// Apple OAuth.
    AppleOAuth,

    /// GitHub OAuth.
    GithubOAuth,

    /// Google OAuth.
    GoogleOAuth,

    /// Microsoft OAuth.
    MicrosoftOAuth,
}
