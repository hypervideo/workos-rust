use serde::{Deserialize, Serialize};

use crate::user_management::User;

/// [WorkOS Docs: `user.created` event](https://workos.com/docs/events/user).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserCreatedEvent(pub User);
