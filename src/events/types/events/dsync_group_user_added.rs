use serde::{Deserialize, Serialize};

use crate::directory_sync::{DirectoryGroup, DirectoryUser};

/// [WorkOS Docs: `dsync.group.user_added` event](https://workos.com/docs/events/directory-sync).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DsyncGroupUserAddedEvent {
    /// The ID of the directory.
    pub directory_id: String,

    /// The directory user.
    pub user: DirectoryUser,

    /// The directory group.
    pub group: DirectoryGroup,
}
