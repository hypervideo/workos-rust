use serde::{Deserialize, Serialize};

use crate::roles::RoleEvent;

/// [WorkOS Docs: `role.created` event](https://workos.com/docs/events/role).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleCreatedEvent(pub RoleEvent);
