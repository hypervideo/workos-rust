use serde::{Deserialize, Serialize};

use crate::directory_sync::DirectoryEvent;

/// [WorkOS Docs: `dsync.deleted` event](https://workos.com/docs/events/directory-sync).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DsyncDeletedEvent(pub DirectoryEvent);
