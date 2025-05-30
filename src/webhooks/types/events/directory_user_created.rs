use serde::Deserialize;

use crate::directory_sync::DirectoryUser;

/// [WorkOS Docs: `dsync.user.created` Webhook](https://workos.com/docs/reference/webhooks/directory-user#webhooks-dsync.user.created)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct DirectoryUserCreatedWebhook(pub DirectoryUser);

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
    fn it_deserializes_a_directory_user_created_webhook() {
        let webhook: Webhook = serde_json::from_str(
            &json!({
              "id": "wh_07FKJ843CVE8F7BXQSPFH0M53V",
              "data": {
                "id": "directory_user_01E1X1B89NH8Z3SDFJR4H7RGX7",
                "directory_id": "directory_01ECAZ4NV9QMV47GW873HDCX74",
                "organization_id": "org_01EZTR6WYX1A0DSE2CYMGXQ24Y",
                "idp_id": "8931",
                "emails": [{
                  "primary": true,
                  "type": "work",
                  "value": "veda@foo-corp.com"
                }],
                "first_name": "Lela",
                "last_name": "Block",
                "username": "veda@foo-corp.com",
                "state": "active",
                "created_at": "2021-06-25T19:07:33.155Z",
                "updated_at": "2021-06-25T19:07:33.155Z",
                "custom_attributes": {
                  "department": "Engineering"
                },
                "raw_attributes": {"idp_id": "8931"}
              },
              "event": "dsync.user.created"
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
                id: WebhookId::from("wh_07FKJ843CVE8F7BXQSPFH0M53V"),
                event: WebhookEvent::DirectoryUserCreated(DirectoryUserCreatedWebhook(
                    DirectoryUser {
                        id: DirectoryUserId::from("directory_user_01E1X1B89NH8Z3SDFJR4H7RGX7"),
                        state: KnownOrUnknown::Known(DirectoryUserState::Active),
                        timestamps: Timestamps {
                            created_at: Timestamp::try_from("2021-06-25T19:07:33.155Z").unwrap(),
                            updated_at: Timestamp::try_from("2021-06-25T19:07:33.155Z").unwrap()
                        },
                        idp_id: "8931".to_string(),
                        directory_id: DirectoryId::from("directory_01ECAZ4NV9QMV47GW873HDCX74"),
                        organization_id: Some(OrganizationId::from(
                            "org_01EZTR6WYX1A0DSE2CYMGXQ24Y"
                        )),
                        username: Some("veda@foo-corp.com".to_string()),
                        emails: vec![DirectoryUserEmail {
                            primary: Some(true),
                            r#type: Some("work".to_string()),
                            value: Some("veda@foo-corp.com".to_string())
                        }],
                        first_name: Some("Lela".to_string()),
                        last_name: Some("Block".to_string()),
                        custom_attributes: expected_custom_attributes,
                        raw_attributes: RawAttributes(expected_raw_attributes),
                    }
                ))
            }
        )
    }
}
