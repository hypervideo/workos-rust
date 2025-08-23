use serde::{Deserialize, Serialize};

use crate::sso::Connection;

/// [WorkOS Docs: `connection.deactivated` event](https://workos.com/docs/events/connection).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectionDeactivatedEvent(pub Connection);
