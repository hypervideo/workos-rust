use serde::{Deserialize, Serialize};

use crate::organizations::OrganizationDomain;

/// [WorkOS Docs: `organization_domain.verified` event](https://workos.com/docs/events/organization-domain).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationDomainVerifiedEvent(pub OrganizationDomain);
