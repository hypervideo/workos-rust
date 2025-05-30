use serde::Deserialize;

use crate::directory_sync::DirectoryUser;

/// [WorkOS Docs: `dsync.user.deleted` Webhook](https://workos.com/docs/reference/webhooks/directory-user#webhooks-dsync.user.deleted)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct DirectoryUserDeletedWebhook(pub DirectoryUser);

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
    fn it_deserializes_a_directory_user_deleted_webhook() {
        let webhook: Webhook = serde_json::from_str(
            &json!({
              "id": "wh_09FKJ843CVE8F7BXQSPFH0M53V",
              "event": "dsync.user.deleted",
              "data": {
                "object": "directory_user",
                "directory_id": "directory_01E1X194NTJ3PYMAY79DYV0F0P",
                "organization_id": "org_01EZTR6WYX1A0DSE2CYMGXQ24Y",
                "id": "directory_user_01E1X1B89NH8Z3SDFJR4H7RGX7",
                "idp_id": "8931",
                "first_name": "Veda",
                "last_name": "Block",
                "username": "veda@foo-corp.com",
                "emails": [
                  {
                    "type": "work",
                    "value": "veda@foo-corp.com",
                    "primary": true
                  }
                ],
                "state": "suspended",
                "created_at": "2021-06-25T19:07:33.155Z",
                "updated_at": "2021-06-25T19:07:33.155Z",
                "raw_attributes": {
                  "idp_id": "8931"
                },
                "custom_attributes": {"department":"Engineering"}
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

        assert_eq!(
            webhook,
            Webhook {
                id: WebhookId::from("wh_09FKJ843CVE8F7BXQSPFH0M53V"),
                event: WebhookEvent::DirectoryUserDeleted(DirectoryUserDeletedWebhook(
                    DirectoryUser {
                        id: DirectoryUserId::from("directory_user_01E1X1B89NH8Z3SDFJR4H7RGX7"),
                        state: KnownOrUnknown::Known(DirectoryUserState::Suspended),
                        timestamps: Timestamps {
                            created_at: Timestamp::try_from("2021-06-25T19:07:33.155Z").unwrap(),
                            updated_at: Timestamp::try_from("2021-06-25T19:07:33.155Z").unwrap()
                        },
                        idp_id: "8931".to_string(),
                        directory_id: DirectoryId::from("directory_01E1X194NTJ3PYMAY79DYV0F0P"),
                        organization_id: Some(OrganizationId::from(
                            "org_01EZTR6WYX1A0DSE2CYMGXQ24Y"
                        )),
                        username: Some("veda@foo-corp.com".to_string()),
                        emails: vec![DirectoryUserEmail {
                            primary: Some(true),
                            r#type: Some("work".to_string()),
                            value: Some("veda@foo-corp.com".to_string())
                        }],
                        first_name: Some("Veda".to_string()),
                        last_name: Some("Block".to_string()),
                        custom_attributes: expected_custom_attributes,
                        raw_attributes: RawAttributes(expected_raw_attributes),
                    }
                ))
            }
        )
    }
}
