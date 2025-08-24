use serde::{Deserialize, Serialize};

use crate::user_management::PasswordResetEvent;

/// [WorkOS Docs: `password_reset.created` event](https://workos.com/docs/events/password-reset).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasswordResetCreatedEvent(pub PasswordResetEvent);
