use serde::{Deserialize, Serialize};

use crate::user_management::OrganizationMembership;

/// [WorkOS Docs: `organization_membership.created` event](https://workos.com/docs/events/organization-membership).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationMembershipCreatedEvent(pub OrganizationMembership);
