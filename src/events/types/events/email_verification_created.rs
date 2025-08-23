use serde::{Deserialize, Serialize};

use crate::user_management::EmailVerificationEvent;

/// [WorkOS Docs: `email_verification.created` event](https://workos.com/docs/events/email-verification).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmailVerificationCreatedEvent(pub EmailVerificationEvent);
