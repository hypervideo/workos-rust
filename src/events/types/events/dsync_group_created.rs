use serde::{Deserialize, Serialize};

use crate::directory_sync::DirectoryGroup;

/// [WorkOS Docs: `dsync.user.created` event](https://workos.com/docs/events/directory-sync).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DsyncGroupCreatedEvent(pub DirectoryGroup);
