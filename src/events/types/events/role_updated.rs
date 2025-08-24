use serde::{Deserialize, Serialize};

use crate::roles::RoleEvent;

/// [WorkOS Docs: `role.updated` event](https://workos.com/docs/events/role).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleUpdatedEvent(pub RoleEvent);
