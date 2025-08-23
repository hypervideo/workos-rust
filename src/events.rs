//! A module for interacting with the WorkOS Events API.
//!
//! [WorkOS Docs: Events Guide](https://workos.com/docs/events/guide)

mod operations;
mod types;

pub use operations::*;
pub use types::*;

use crate::WorkOs;

/// Events.
///
/// [WorkOS Docs: Events Guide](https://workos.com/docs/events/guide)
pub struct Events<'a> {
    workos: &'a WorkOs,
}

impl<'a> Events<'a> {
    /// Returns a new [`Events`] instance for the provided WorkOS client.
    pub fn new(workos: &'a WorkOs) -> Self {
        Self { workos }
    }
}
