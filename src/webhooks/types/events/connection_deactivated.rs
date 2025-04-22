use serde::Deserialize;

use crate::sso::Connection;

/// [WorkOS Docs: `connection.deactivated` Webhook](https://workos.com/docs/reference/webhooks/connection#webhooks-sso.connection.deactivated)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct ConnectionDeactivatedWebhook(pub Connection);

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::organizations::OrganizationId;
    use crate::sso::{ConnectionId, ConnectionState, ConnectionType};
    use crate::webhooks::{Webhook, WebhookEvent, WebhookId};
    use crate::{KnownOrUnknown, Timestamp, Timestamps};

    use super::*;

    #[test]
    fn it_deserializes_a_connection_deactivated_webhook() {
        let webhook: Webhook = serde_json::from_str(
            &json!({
              "id": "wh_01G69A99BZ7X4T4XZ809A630Y7",
              "event": "connection.deactivated",
              "data": {
                "object": "connection",
                "id": "conn_01EHWNC0FCBHZ3BJ7EGKYXK0E6",
                "organization_id": "org_01EHWNCE74X7JSDV0X3SZ3KJNY",
                "external_key": "3QMR4u0Tok6SgwY2AWG6u6mkQ",
                "connection_type": "OktaSAML",
                "name": "Foo Corp's Connection",
                "state": "inactive",
                "status": "unlinked",
                "domains": [
                  {
                    "object": "connection_domain",
                    "id": "conn_domain_01EHWNFTAFCF3CQAE5A9Q0P1YB",
                    "domain": "foo-corp.com"
                  }
                ],
                "created_at": "2021-06-25T19:07:33.155Z",
                "updated_at": "2021-06-25T19:07:33.155Z"
              }
            })
            .to_string(),
        )
        .unwrap();

        assert_eq!(
            webhook,
            Webhook {
                id: WebhookId::from("wh_01G69A99BZ7X4T4XZ809A630Y7"),
                event: WebhookEvent::ConnectionDeactivated(ConnectionDeactivatedWebhook(
                    Connection {
                        id: ConnectionId::from("conn_01EHWNC0FCBHZ3BJ7EGKYXK0E6"),
                        organization_id: Some(OrganizationId::from(
                            "org_01EHWNCE74X7JSDV0X3SZ3KJNY"
                        )),
                        r#type: KnownOrUnknown::Known(ConnectionType::OktaSaml),
                        name: "Foo Corp's Connection".to_string(),
                        state: KnownOrUnknown::Known(ConnectionState::Inactive),
                        timestamps: Timestamps {
                            created_at: Timestamp::try_from("2021-06-25T19:07:33.155Z").unwrap(),
                            updated_at: Timestamp::try_from("2021-06-25T19:07:33.155Z").unwrap()
                        }
                    }
                ))
            }
        )
    }
}
