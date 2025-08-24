use serde::{Deserialize, Serialize};

use crate::roles::RoleEvent;

/// [WorkOS Docs: `role.deleted` event](https://workos.com/docs/events/role).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleDeletedEvent(pub RoleEvent);
