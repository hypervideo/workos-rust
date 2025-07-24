//! A module for interacting with the WorkOS Single Sign-On (SSO) API.
//!
//! [WorkOS Docs: SSO Guide](https://workos.com/docs/sso/guide)

mod operations;
mod types;

pub use operations::*;
pub use types::*;

use crate::WorkOs;

/// Single Sign-On (SSO).
///
/// [WorkOS Docs: SSO Guide](https://workos.com/docs/sso/guide)
pub struct Sso<'a> {
    workos: &'a WorkOs<'a>,
}

impl<'a> Sso<'a> {
    /// Returns a new [`Sso`] instance for the provided WorkOS client.
    pub fn new(workos: &'a WorkOs) -> Self {
        Self { workos }
    }
}
