use serde::{Deserialize, Serialize};

use crate::sso::Connection;

/// [WorkOS Docs: `connection.activated` event](https://workos.com/docs/events/connection).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectionActivatedEvent(pub Connection);
