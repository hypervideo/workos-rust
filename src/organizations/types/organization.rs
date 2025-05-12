use derive_more::{Deref, Display, From};
use serde::{Deserialize, Serialize};

use crate::Timestamps;

/// The ID of an [`Organization`].
#[derive(
    Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[from(forward)]
pub struct OrganizationId(String);

/// The ID and name of an [`Organization`].
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct OrganizationIdAndName {
    /// The ID of the organization.
    pub id: OrganizationId,

    /// The name of the organization.
    pub name: String,
}

/// [WorkOS Docs: Organization](https://workos.com/docs/reference/organization)
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Organization {
    /// The ID of the organization.
    pub id: OrganizationId,

    /// The name of the organization.
    pub name: String,

    /// Whether the connections within this organization should allow profiles
    /// that do not have a domain that is present in the set of the organization's
    /// user email domains.
    ///
    /// See [here](https://workos.com/docs/sso/guide/frequently-asked-questions#allow-profiles-outside-organization)
    /// for more details.
    pub allow_profiles_outside_organization: bool,

    /// The list of user email domains for the organization.
    pub domains: Vec<OrganizationDomain>,

    /// The timestamps for the organization.
    #[serde(flatten)]
    pub timestamps: Timestamps,
}

/// The ID of an [`OrganizationDomain`].
#[derive(
    Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[from(forward)]
pub struct OrganizationDomainId(String);

/// [WorkOS Docs: Organization Domain](https://workos.com/docs/reference/organization-domain)
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationDomain {
    /// The ID of the organization domain.
    pub id: OrganizationDomainId,

    /// The domain.
    pub domain: String,
}
