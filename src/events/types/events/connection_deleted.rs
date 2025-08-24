use serde::{Deserialize, Serialize};

use crate::sso::Connection;

/// [WorkOS Docs: `connection.deleted` event](https://workos.com/docs/events/connection).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectionDeletedEvent(pub Connection);
