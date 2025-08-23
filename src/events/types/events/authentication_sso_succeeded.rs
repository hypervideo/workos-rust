use serde::{Deserialize, Serialize};

use crate::user_management::AuthenticationEvent;

/// [WorkOS Docs: `authentication.sso_succeeded` event](https://workos.com/docs/events/authentication).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticationSsoSucceededEvent(pub AuthenticationEvent);
