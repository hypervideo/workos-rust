use serde::{Deserialize, Serialize};

use crate::user_management::MagicAuthEvent;

/// [WorkOS Docs: `magic_auth.created` event](https://workos.com/docs/events/magic-auth).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MagicAuthCreatedEvent(pub MagicAuthEvent);
