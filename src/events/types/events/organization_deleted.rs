use serde::{Deserialize, Serialize};

use crate::organizations::Organization;

/// [WorkOS Docs: `organization.deleted` event](https://workos.com/docs/events/organization).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationDeletedEvent(pub Organization);
