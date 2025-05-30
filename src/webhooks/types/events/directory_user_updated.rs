use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

use crate::directory_sync::DirectoryUser;

/// A [`DirectoryUser`] with their previous attributes.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct DirectoryUserWithPreviousAttributes {
    /// The directory user.
    #[serde(flatten)]
    pub directory_user: DirectoryUser,

    /// The previous values for any attributes that were updated.
    pub previous_attributes: HashMap<String, Value>,
}

/// [WorkOS Docs: `dsync.user.updated` Webhook](https://workos.com/docs/reference/webhooks/directory-user#webhooks-dsync.user.updated)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct DirectoryUserUpdatedWebhook(pub DirectoryUserWithPreviousAttributes);

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use serde_json::{Value, json};

    use crate::directory_sync::{
        DirectoryId, DirectoryUserEmail, DirectoryUserId, DirectoryUserState,
    };

    use crate::organizations::OrganizationId;
    use crate::webhooks::{Webhook, WebhookEvent, WebhookId};
    use crate::{KnownOrUnknown, RawAttributes, Timestamp, Timestamps};

    use super::*;

    #[test]
    fn it_deserializes_a_directory_user_updated_webhook() {
        let webhook: Webhook = serde_json::from_str(
            &json!(
            {
                "id": "wh_08FKJ843CVE8F7BXQSPFH0M53V",
                "event": "dsync.user.updated",
                "data": {
                  "object": "directory_user",
                  "directory_id": "directory_01E1X194NTJ3PYMAY79DYV0F0P",
                  "organization_id": "org_01EZTR6WYX1A0DSE2CYMGXQ24Y",
                  "id": "directory_user_01E1X1B89NH8Z3SDFJR4H7RGX7",
                  "idp_id": "8931",
                  "first_name": "Veda",
                  "last_name": "Block",
                  "username": "veda@example.com",
                  "emails": [
                    {
                      "type": "work",
                      "value": "veda@example.com",
                      "primary": true
                    }
                  ],
                  "state": "suspended",
                  "created_at": "2021-06-25T19:07:33.155Z",
                  "updated_at": "2021-06-25T19:07:33.155Z",
                  "raw_attributes": {
                    "idp_id": "8931",
                  },
                  "custom_attributes": {
                    "department": "Engineering",
                  },
                  "previous_attributes": {
                    "lastName": "Cube",
                  }
                }
              })
            .to_string(),
        )
        .unwrap();

        let mut expected_custom_attributes = HashMap::new();
        expected_custom_attributes.insert(
            "department".to_string(),
            Value::String("Engineering".to_string()),
        );

        let mut expected_raw_attributes = HashMap::new();
        expected_raw_attributes.insert("idp_id".to_string(), Value::String("8931".to_string()));

        let mut expected_previous_attributes = HashMap::new();
        expected_previous_attributes
            .insert("lastName".to_string(), Value::String("Cube".to_string()));

        assert_eq!(
            webhook,
            Webhook {
                id: WebhookId::from("wh_08FKJ843CVE8F7BXQSPFH0M53V"),
                event: WebhookEvent::DirectoryUserUpdated(DirectoryUserUpdatedWebhook(
                    DirectoryUserWithPreviousAttributes {
                        directory_user: DirectoryUser {
                            id: DirectoryUserId::from("directory_user_01E1X1B89NH8Z3SDFJR4H7RGX7"),
                            state: KnownOrUnknown::Known(DirectoryUserState::Suspended),
                            timestamps: Timestamps {
                                created_at: Timestamp::try_from("2021-06-25T19:07:33.155Z")
                                    .unwrap(),
                                updated_at: Timestamp::try_from("2021-06-25T19:07:33.155Z")
                                    .unwrap()
                            },
                            idp_id: "8931".to_string(),
                            directory_id: DirectoryId::from("directory_01E1X194NTJ3PYMAY79DYV0F0P"),
                            organization_id: Some(OrganizationId::from(
                                "org_01EZTR6WYX1A0DSE2CYMGXQ24Y"
                            )),
                            username: Some("veda@example.com".to_string()),
                            emails: vec![DirectoryUserEmail {
                                primary: Some(true),
                                r#type: Some("work".to_string()),
                                value: Some("veda@example.com".to_string())
                            }],
                            first_name: Some("Veda".to_string()),
                            last_name: Some("Block".to_string()),
                            custom_attributes: expected_custom_attributes,
                            raw_attributes: RawAttributes(expected_raw_attributes),
                        },
                        previous_attributes: expected_previous_attributes
                    }
                ))
            }
        )
    }
}
