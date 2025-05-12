use serde::Deserialize;

/// Possible methods the user can use to authenticate.
#[derive(Debug, Deserialize)]
pub struct AuthenticateMethods {
    /// Whether or not Sign in with Apple is enabled for the organization.
    pub apple_oauth: bool,

    /// Whether or not GitHub OAuth is enabled for the organization.
    pub github_oauth: bool,

    /// Whether or not Google OAuth is enabled for the organization.
    pub google_oauth: bool,

    /// Whether or not Magic Auth is enabled for the organization.
    pub magic_auth: bool,

    /// Whether or not Microsoft OAuth is enabled for the organization.
    pub microsoft_auth: bool,

    /// Whether or not password authentication is enabled for the organization.
    pub password: bool,
}
