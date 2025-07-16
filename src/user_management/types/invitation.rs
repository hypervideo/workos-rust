use derive_more::{Deref, Display, From};
use serde::{Deserialize, Serialize};
use url::Url;
use crate::{ Timestamp};
use crate::organizations::OrganizationId;
use crate::user_management::UserId;

/// The ID of a [`Invitation`].
#[derive(
    Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[from(forward)]
pub struct InvitationId(String);

/// The token of an [`Invitation`].
#[derive(
    Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[from(forward)]
pub struct InvitationToken(String);

/// [WorkOS Docs: User](https://workos.com/docs/reference/user-management/invitation)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Invitation {
    /// The unique ID of the invitation.
    pub id: InvitationId,

    /// The email address of the user.
    pub email: String,

    /// The state of the invitation.
    pub state: String,

    /// The timestamp when the invitation was accepted.
    pub accepted_at: Option<Timestamp>,

    /// The timestamp when the invitation was revoked.
    pub revoked_at: Option<Timestamp>,

    /// The timestamp when the invitation expires.
    pub expires_at: Timestamp,

    /// The token for the invitation.
    pub token: String,

    /// The URL to accept the invitation.
    pub accept_invitation_url: Url,

    /// The organization ID that the invitation is for.
    pub organization_id: OrganizationId,

    /// The user ID of the user who invited the recipient.
    pub inviter_user_id: UserId,

    /// When the invitation was created.
    pub created_at: Timestamp,

    /// When the invitation was last updated.
    pub updated_at: Timestamp
}
