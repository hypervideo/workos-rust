use derive_more::{Deref, Display, From};
use serde::{Deserialize, Serialize};

use crate::organizations::OrganizationId;
use crate::user_management::UserId;
use crate::Timestamps;

/// The ID of an [`OrganizationMembership`].
#[derive(
    Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[from(forward)]
pub struct OrganizationMembershipId(String);

/// The status of an organization membership.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrganizationMembershipStatus {
    /// The membership is active.
    Active,
    /// The membership is inactive.
    Inactive,
    /// The membership is pending.
    Pending,
}

/// The role of an organization membership.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationMembershipRole {
    /// The slug of the role.
    pub slug: String,
}

/// [WorkOS Docs: Organization Membership](https://workos.com/docs/reference/authkit/organization-membership)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrganizationMembership {
    /// The unique ID of the organization membership.
    pub id: OrganizationMembershipId,

    /// The unique ID of the user.
    pub user_id: UserId,

    /// The unique ID of the organization.
    pub organization_id: OrganizationId,

    /// The role of the user in the organization.
    pub role: OrganizationMembershipRole,

    /// The status of the membership.
    pub status: OrganizationMembershipStatus,

    /// The timestamps for the organization membership.
    #[serde(flatten)]
    pub timestamps: Timestamps,
}