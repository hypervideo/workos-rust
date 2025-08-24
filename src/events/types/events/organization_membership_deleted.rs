use serde::{Deserialize, Serialize};

use crate::user_management::OrganizationMembership;

/// [WorkOS Docs: `organization_membership.deleted` event](https://workos.com/docs/events/organization-membership).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationMembershipDeletedEvent(pub OrganizationMembership);
