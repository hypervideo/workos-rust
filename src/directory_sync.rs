//! A module for interacting with the WorkOS Directory Sync API.
//!
//! [WorkOS Docs: Directory Sync Guide](https://workos.com/docs/directory-sync/guide)

mod operations;
mod types;

pub use operations::*;
pub use types::*;

use crate::WorkOs;

/// Directory Sync.
///
/// [WorkOS Docs: Directory Sync Guide](https://workos.com/docs/directory-sync/guide)
pub struct DirectorySync<'a> {
    workos: &'a WorkOs<'a>,
}

impl<'a> DirectorySync<'a> {
    /// Returns a new [`DirectorySync`] instance for the provided WorkOS client.
    pub fn new(workos: &'a WorkOs) -> Self {
        Self { workos }
    }
}
