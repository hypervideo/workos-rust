use serde::{Deserialize, Serialize};

use crate::directory_sync::DirectoryUser;

/// [WorkOS Docs: `dsync.user.updated` event](https://workos.com/docs/events/directory-sync).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DsyncUserUpdatedEvent(pub DirectoryUser);
