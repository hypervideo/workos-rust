use serde::{Deserialize, Serialize};

use crate::Timestamps;

/// The slug of a [`Role`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleSlug {
    /// A unique key to reference the role.
    pub slug: String,
}

/// [WorkOS Docs: Role events](https://workos.com/docs/events/role)
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleEvent {
    /// A unique key to reference the role.
    pub slug: String,

    /// A list of permission slugs assigned to the role.
    pub permissions: Vec<String>,

    /// The timestamps for the role.
    #[serde(flatten)]
    pub timestamps: Timestamps,
}
