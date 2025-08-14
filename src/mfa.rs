//! A module for interacting with the WorkOS Multi-factor Authentication (MFA) API.
//!
//! [WorkOS Docs: MFA Guide](https://workos.com/docs/mfa/guide)

mod operations;
mod types;

pub use operations::*;
pub use types::*;

use crate::WorkOs;

/// Multi-factor Authentication (MFA).
///
/// [WorkOS Docs: MFA Guide](https://workos.com/docs/mfa/guide)
pub struct Mfa<'a> {
    workos: &'a WorkOs<'a>,
}

impl<'a> Mfa<'a> {
    /// Returns a new [`Mfa`] instance for the provided WorkOS client.
    pub fn new(workos: &'a WorkOs) -> Self {
        Self { workos }
    }
}
