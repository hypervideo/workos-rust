//! A module for interacting with the WorkOS Admin Portal.
//!
//! [WorkOS Docs: Admin Portal Guide](https://workos.com/docs/admin-portal/guide)

mod operations;

pub use operations::*;

use crate::WorkOs;

/// Admin Portal.
///
/// [WorkOS Docs: Admin Portal Guide](https://workos.com/docs/admin-portal/guide)
pub struct AdminPortal<'a> {
    workos: &'a WorkOs<'a>,
}

impl<'a> AdminPortal<'a> {
    /// Returns a new [`AdminPortal`] instance for the provided WorkOS client.
    pub fn new(workos: &'a WorkOs) -> Self {
        Self { workos }
    }
}
