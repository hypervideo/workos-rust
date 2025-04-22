use serde::Deserialize;

use crate::webhooks::Directory;

/// [WorkOS Docs: `dsync.deleted` Webhook](https://workos.com/docs/reference/webhooks/directory#webhooks-dsync.deleted)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct DirectoryDeletedWebhook(pub Directory);

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::directory_sync::{DirectoryId, DirectoryType};
    use crate::organizations::OrganizationId;
    use crate::webhooks::{
        Directory, DirectoryDeletedWebhook, DirectoryState, Webhook, WebhookEvent, WebhookId,
    };
    use crate::{KnownOrUnknown, Timestamp, Timestamps};

    #[test]
    fn it_deserializes_a_directory_deleted_webhook() {
        let webhook: Webhook = serde_json::from_str(
            &json!({
              "id": "wh_03FKJ843CVE8F7BXQSPFH0M53V",
              "data": {
                "object": "directory",
                "id": "directory_01EHWNC0FCBHZ3BJ7EGKYXK0E6",
                "organization_id": "org_01EZTR6WYX1A0DSE2CYMGXQ24Y",
                "type": "generic scim v2.0",
                "state": "deleting",
                "name": "Foo Corp's Directory",
                "created_at": "2021-06-25T19:07:33.155Z",
                "updated_at": "2021-06-25T19:07:33.155Z"
              },
              "event": "dsync.deleted"
            })
            .to_string(),
        )
        .unwrap();

        assert_eq!(
            webhook,
            Webhook {
                id: WebhookId::from("wh_03FKJ843CVE8F7BXQSPFH0M53V"),
                event: WebhookEvent::DirectoryDeleted(DirectoryDeletedWebhook(Directory {
                    id: DirectoryId::from("directory_01EHWNC0FCBHZ3BJ7EGKYXK0E6"),
                    organization_id: Some(OrganizationId::from("org_01EZTR6WYX1A0DSE2CYMGXQ24Y")),
                    r#type: KnownOrUnknown::Known(DirectoryType::GenericScimV2_0),
                    name: "Foo Corp's Directory".to_string(),
                    state: KnownOrUnknown::Known(DirectoryState::Deleting),
                    timestamps: Timestamps {
                        created_at: Timestamp::try_from("2021-06-25T19:07:33.155Z").unwrap(),
                        updated_at: Timestamp::try_from("2021-06-25T19:07:33.155Z").unwrap()
                    },
                }))
            }
        )
    }
}
