use serde::{Deserialize, Serialize};

use crate::user_management::AuthenticationEvent;

/// [WorkOS Docs: `authentication.password_failed` event](https://workos.com/docs/events/authentication).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticationPasswordFailedEvent(pub AuthenticationEvent);
