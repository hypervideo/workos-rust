use serde::{Deserialize, Serialize};

use crate::user_management::Session;

/// [WorkOS Docs: `session.created` event](https://workos.com/docs/events/session).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionCreatedEvent(pub Session);
