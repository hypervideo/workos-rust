use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

use crate::directory_sync::DirectoryGroup;

/// A [`DirectoryGroup`] with its previous attributes.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct DirectoryGroupWithPreviousAttributes {
    /// The directory group.
    #[serde(flatten)]
    pub directory_group: DirectoryGroup,

    /// The previous values for any attributes that were updated.
    pub previous_attributes: HashMap<String, Value>,
}

/// [WorkOS Docs: `dsync.group.updated` Webhook](https://workos.com/docs/reference/webhooks/directory-group#webhooks-dsync.group.updated)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct DirectoryGroupUpdatedWebhook(pub DirectoryGroupWithPreviousAttributes);

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
    fn it_deserializes_a_directory_group_updated_webhook() {
        let webhook: Webhook = serde_json::from_str(
            &json!(
            {
                "id": "wh_01G69AB8ZFCNY91AV8850C003X",
                "event": "dsync.group.updated",
                "data": {
                  "object": "directory_group",
                  "id": "directory_group_01E1X1B89NH8Z3SDFJR4H7RGX7",
                  "directory_id": "directory_01E1X194NTJ3PYMAY79DYV0F0P",
                  "organization_id": "org_01EZTR6WYX1A0DSE2CYMGXQ24Y",
                  "idp_id": "02grqrue4294w24",
                  "name": "Developers",
                  "created_at": "2021-06-25T19:07:33.155Z",
                  "updated_at": "2021-06-25T19:07:33.155Z",
                  "raw_attributes": {
                    "id": "8931"
                  },
                  "previous_attributes": {
                    "name": "Software Engineers"
                  }
                }
              })
            .to_string(),
        )
        .unwrap();

        let mut expected_raw_attributes = HashMap::new();
        expected_raw_attributes.insert("id".to_string(), Value::String("8931".to_string()));

        let mut expected_previous_attributes = HashMap::new();
        expected_previous_attributes.insert(
            "name".to_string(),
            Value::String("Software Engineers".to_string()),
        );

        assert_eq!(
            webhook,
            Webhook {
                id: WebhookId::from("wh_01G69AB8ZFCNY91AV8850C003X"),
                event: WebhookEvent::DirectoryGroupUpdated(DirectoryGroupUpdatedWebhook(
                    DirectoryGroupWithPreviousAttributes {
                        directory_group: DirectoryGroup {
                            id: DirectoryGroupId::from(
                                "directory_group_01E1X1B89NH8Z3SDFJR4H7RGX7"
                            ),
                            idp_id: "02grqrue4294w24".to_string(),
                            directory_id: DirectoryId::from("directory_01E1X194NTJ3PYMAY79DYV0F0P"),
                            organization_id: Some(OrganizationId::from(
                                "org_01EZTR6WYX1A0DSE2CYMGXQ24Y"
                            )),
                            name: "Developers".to_string(),
                            timestamps: Timestamps {
                                created_at: Timestamp::try_from("2021-06-25T19:07:33.155Z")
                                    .unwrap(),
                                updated_at: Timestamp::try_from("2021-06-25T19:07:33.155Z")
                                    .unwrap(),
                            },
                            raw_attributes: RawAttributes(expected_raw_attributes)
                        },
                        previous_attributes: expected_previous_attributes
                    }
                ))
            }
        )
    }
}
