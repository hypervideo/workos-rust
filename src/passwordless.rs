//! A module for working with passwordless authentication, namely Magic Link.
//!
//! [WorkOS Docs: Magic Link Guide](https://workos.com/docs/magic-link/guide)

mod operations;
mod types;

pub use operations::*;
pub use types::*;

use crate::WorkOs;

/// Passwordless (Magic Link).
///
/// [WorkOS Docs: Magic Link Guide](https://workos.com/docs/magic-link/guide)
pub struct Passwordless<'a> {
    workos: &'a WorkOs<'a>,
}

impl<'a> Passwordless<'a> {
    /// Returns a new [`Passwordless`] instance for the provided WorkOS client.
    pub fn new(workos: &'a WorkOs) -> Self {
        Self { workos }
    }
}
