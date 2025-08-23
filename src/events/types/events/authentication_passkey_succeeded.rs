use serde::{Deserialize, Serialize};

use crate::user_management::AuthenticationEvent;

/// [WorkOS Docs: `authentication.passkey_succeeded` event](https://workos.com/docs/events/authentication).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticationPasskeySucceededEvent(pub AuthenticationEvent);
