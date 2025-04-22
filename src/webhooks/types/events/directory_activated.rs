use serde::Deserialize;

use crate::webhooks::Directory;

/// [WorkOS Docs: `dsync.activated` Webhook](https://workos.com/docs/reference/webhooks/directory#webhooks-dsync.activated)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct DirectoryActivatedWebhook(pub Directory);

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::directory_sync::{DirectoryId, DirectoryType};
    use crate::organizations::OrganizationId;
    use crate::webhooks::{
        Directory, DirectoryActivatedWebhook, DirectoryState, Webhook, WebhookEvent, WebhookId,
    };
    use crate::{KnownOrUnknown, Timestamp, Timestamps};

    #[test]
    fn it_deserializes_a_directory_activated_webhook() {
        let webhook: Webhook = serde_json::from_str(
            &json!({
              "id": "wh_01FKJ843CVE8F7BXQSPFH0M53V",
              "data": {
                "object": "directory",
                "name": "Foo Corp's Directory",
                "organization_id": "org_01EZTR6WYX1A0DSE2CYMGXQ24Y",
                "id": "directory_01EHWNC0FCBHZ3BJ7EGKYXK0E6",
                "state": "active",
                "type": "generic scim v2.0",
                "created_at": "2021-06-25T19:07:33.155Z",
                "updated_at": "2021-06-25T19:07:33.155Z",
              },
              "event": "dsync.activated"
            })
            .to_string(),
        )
        .unwrap();

        assert_eq!(
            webhook,
            Webhook {
                id: WebhookId::from("wh_01FKJ843CVE8F7BXQSPFH0M53V"),
                event: WebhookEvent::DirectoryActivated(DirectoryActivatedWebhook(Directory {
                    id: DirectoryId::from("directory_01EHWNC0FCBHZ3BJ7EGKYXK0E6"),
                    organization_id: Some(OrganizationId::from("org_01EZTR6WYX1A0DSE2CYMGXQ24Y")),
                    r#type: KnownOrUnknown::Known(DirectoryType::GenericScimV2_0),
                    name: "Foo Corp's Directory".to_string(),
                    state: KnownOrUnknown::Known(DirectoryState::Active),
                    timestamps: Timestamps {
                        created_at: Timestamp::try_from("2021-06-25T19:07:33.155Z").unwrap(),
                        updated_at: Timestamp::try_from("2021-06-25T19:07:33.155Z").unwrap()
                    },
                }))
            }
        )
    }
}
