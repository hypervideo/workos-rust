use serde::{Deserialize, Serialize};

use crate::user_management::User;

/// [WorkOS Docs: `user.deleted` event](https://workos.com/docs/events/user).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserDeletedEvent(pub User);
