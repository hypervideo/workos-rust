use serde::Deserialize;

use crate::directory_sync::DirectoryGroup;

/// [WorkOS Docs: `dsync.group.created` Webhook](https://workos.com/docs/reference/webhooks/directory-group#webhooks-dsync.group.created)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct DirectoryGroupCreatedWebhook(pub DirectoryGroup);

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use serde_json::{Value, json};

    use crate::directory_sync::{DirectoryGroupId, DirectoryId};

    use crate::organizations::OrganizationId;
    use crate::webhooks::{Webhook, WebhookEvent, WebhookId};
    use crate::{RawAttributes, Timestamp, Timestamps};

    use super::*;

    #[test]
    fn it_deserializes_a_directory_group_created_webhook() {
        let webhook: Webhook = serde_json::from_str(
            &json!(
            {
                "id": "wh_01G69AA11NCGV97NT1D4TYZP2T",
                "event": "dsync.group.created",
                "data": {
                  "object": "directory_group",
                  "id": "directory_group_01E1X5GPMMXF4T1DCERMVEEPVW",
                  "directory_id": "directory_01E1X194NTJ3PYMAY79DYV0F0P",
                  "organization_id": "org_01EZTR6WYX1A0DSE2CYMGXQ24Y",
                  "idp_id": "02grqrue4294w24",
                  "name": "Developers",
                  "created_at": "2021-06-25T19:07:33.155Z",
                  "updated_at": "2021-06-25T19:07:33.155Z",
                  "raw_attributes": {
                    "id": "02grqrue4294w24"
                  }
                }
              })
            .to_string(),
        )
        .unwrap();

        let mut expected_raw_attributes = HashMap::new();
        expected_raw_attributes.insert(
            "id".to_string(),
            Value::String("02grqrue4294w24".to_string()),
        );

        assert_eq!(
            webhook,
            Webhook {
                id: WebhookId::from("wh_01G69AA11NCGV97NT1D4TYZP2T"),
                event: WebhookEvent::DirectoryGroupCreated(DirectoryGroupCreatedWebhook(
                    DirectoryGroup {
                        id: DirectoryGroupId::from("directory_group_01E1X5GPMMXF4T1DCERMVEEPVW"),
                        idp_id: "02grqrue4294w24".to_string(),
                        directory_id: DirectoryId::from("directory_01E1X194NTJ3PYMAY79DYV0F0P"),
                        organization_id: Some(OrganizationId::from(
                            "org_01EZTR6WYX1A0DSE2CYMGXQ24Y"
                        )),
                        name: "Developers".to_string(),
                        timestamps: Timestamps {
                            created_at: Timestamp::try_from("2021-06-25T19:07:33.155Z").unwrap(),
                            updated_at: Timestamp::try_from("2021-06-25T19:07:33.155Z").unwrap(),
                        },
                        raw_attributes: RawAttributes(expected_raw_attributes)
                    }
                ))
            }
        )
    }
}
