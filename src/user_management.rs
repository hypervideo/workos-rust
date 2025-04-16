//! A module for interacting with the WorkOS User Management API.
//!
//! [WorkOS Docs: User Management](https://workos.com/docs/user-management)

mod operations;
mod types;

pub use operations::*;
pub use types::*;

use crate::WorkOs;

/// User Management.
///
/// [WorkOS Docs: User Management](https://workos.com/docs/user-management)
pub struct UserManagement<'a> {
    workos: &'a WorkOs,
}

impl<'a> UserManagement<'a> {
    /// Returns a new [`UserManagement`] instance for the provided WorkOS client.
    pub fn new(workos: &'a WorkOs) -> Self {
        Self { workos }
    }
}
