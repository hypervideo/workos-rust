use serde::{Deserialize, Serialize};

use crate::user_management::Session;

/// [WorkOS Docs: `session.revoked` event](https://workos.com/docs/events/session).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionRevokedEvent(pub Session);
