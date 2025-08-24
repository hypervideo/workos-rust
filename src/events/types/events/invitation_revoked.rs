use serde::{Deserialize, Serialize};

use crate::user_management::InvitationEvent;

/// [WorkOS Docs: `invitation.revoked` event](https://workos.com/docs/events/invitation).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvitationRevokedEvent(pub InvitationEvent);
