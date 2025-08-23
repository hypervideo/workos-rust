use derive_more::{Deref, Display, From};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::organizations::OrganizationId;
use crate::user_management::UserId;
use crate::{KnownOrUnknown, Timestamp, Timestamps};

/// The ID of an [`Invitation`].
#[derive(
    Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[from(forward)]
pub struct InvitationId(String);

/// The state of an [`Invitation`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvitationState {
    /// The invitation is pending.
    Pending,

    /// The invitation is accepted.
    Accepted,

    /// The invitation is expired.
    Expired,

    /// The invitation is revoked.
    Revoked,
}

/// The token of an [`Invitation`].
#[derive(
    Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[from(forward)]
pub struct InvitationToken(String);

/// [WorkOS Docs: Invitation](https://workos.com/docs/reference/user-management/invitation)
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Invitation {
    /// The unique ID of the invitation.
    pub id: InvitationId,

    /// The email address of the user.
    pub email: String,

    /// The state of the invitation.
    pub state: KnownOrUnknown<InvitationState, String>,

    /// The timestamp indicating when the invitation was accepted.
    pub accepted_at: Option<Timestamp>,

    /// The timestamp indicating when the invitation was revoked.
    pub revoked_at: Option<Timestamp>,

    /// The timestamp indicating when the invitation expires.
    pub expires_at: Timestamp,

    /// The token for the invitation.
    pub token: InvitationToken,

    /// The URL used to accept the invitation.
    pub accept_invitation_url: Url,

    /// The ID of the organization that the recipient will join.
    pub organization_id: Option<OrganizationId>,

    /// The ID of the user who invited the recipient.
    pub inviter_user_id: Option<UserId>,

    /// The ID of the user who accepted the invitation.
    pub accepted_user_id: Option<UserId>,

    /// The timestamps for the invitation.
    #[serde(flatten)]
    pub timestamps: Timestamps,
}

/// [WorkOS Docs: Invitation events](https://workos.com/docs/events/invitation)
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvitationEvent {
    /// The unique ID of the invitation.
    pub id: InvitationId,

    /// The email address of the user.
    pub email: String,

    /// The state of the invitation.
    pub state: KnownOrUnknown<InvitationState, String>,

    /// The timestamp indicating when the invitation was accepted.
    pub accepted_at: Option<Timestamp>,

    /// The timestamp indicating when the invitation was revoked.
    pub revoked_at: Option<Timestamp>,

    /// The timestamp indicating when the invitation expires.
    pub expires_at: Timestamp,

    /// The ID of the organization that the recipient will join.
    pub organization_id: Option<OrganizationId>,

    /// The ID of the user who invited the recipient.
    pub inviter_user_id: Option<UserId>,

    /// The ID of the user who accepted the invitation.
    pub accepted_user_id: Option<UserId>,

    /// The timestamps for the invitation.
    #[serde(flatten)]
    pub timestamps: Timestamps,
}
