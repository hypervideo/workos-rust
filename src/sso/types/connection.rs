use derive_more::{Deref, Display, From};
use serde::{Deserialize, Serialize};

use crate::organizations::OrganizationId;
use crate::sso::ConnectionType;
use crate::{KnownOrUnknown, Timestamps};

/// The ID of a [`Connection`].
#[derive(
    Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[from(forward)]
pub struct ConnectionId(String);

/// The state of a [`Connection`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionState {
    /// The connection is active.
    Active,

    /// The connection is inactive.
    Inactive,
}

/// [WorkOS Docs: Connection](https://workos.com/docs/reference/sso/connection)
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Connection {
    /// The ID of the connection.
    pub id: ConnectionId,

    /// The ID of the associated [`Organization`](crate::organizations::Organization) for this connection.
    pub organization_id: Option<OrganizationId>,

    /// The type of the connection.
    #[serde(rename = "connection_type")]
    pub r#type: KnownOrUnknown<ConnectionType, String>,

    /// The display name of the connection.
    pub name: String,

    /// The state of the connection.
    pub state: KnownOrUnknown<ConnectionState, String>,

    /// The timestamps for the connection.
    #[serde(flatten)]
    pub timestamps: Timestamps,
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::organizations::OrganizationId;
    use crate::sso::ConnectionType;
    use crate::{KnownOrUnknown, Timestamp, Timestamps};

    use super::{Connection, ConnectionId, ConnectionState};

    #[test]
    fn it_deserializes_a_connection() {
        let connection: Connection = serde_json::from_str(
            &json!({
              "object": "connection",
              "id": "conn_01E4ZCR3C56J083X43JQXF3JK5",
              "organization_id": "org_01EHWNCE74X7JSDV0X3SZ3KJNY",
              "connection_type": "GoogleOAuth",
              "name": "Foo Corp",
              "state": "active",
              "created_at": "2021-06-25T19:07:33.155Z",
              "updated_at": "2021-06-25T19:07:33.155Z",
            })
            .to_string(),
        )
        .unwrap();

        assert_eq!(
            connection,
            Connection {
                id: ConnectionId::from("conn_01E4ZCR3C56J083X43JQXF3JK5"),
                organization_id: Some(OrganizationId::from("org_01EHWNCE74X7JSDV0X3SZ3KJNY")),
                r#type: KnownOrUnknown::Known(ConnectionType::GoogleOauth),
                name: "Foo Corp".to_string(),
                state: KnownOrUnknown::Known(ConnectionState::Active),
                timestamps: Timestamps {
                    created_at: Timestamp::try_from("2021-06-25T19:07:33.155Z").unwrap(),
                    updated_at: Timestamp::try_from("2021-06-25T19:07:33.155Z").unwrap(),
                }
            }
        )
    }

    #[test]
    fn it_deserializes_unknown_connection_types() {
        let connection: Connection = serde_json::from_str(
            &json!({
              "object": "connection",
              "id": "conn_01E4ZCR3C56J083X43JQXF3JK5",
              "organization_id": "org_01EHWNCE74X7JSDV0X3SZ3KJNY",
              "connection_type": "UnknownType",
              "name": "Foo Corp",
              "state": "active",
              "created_at": "2021-06-25T19:07:33.155Z",
              "updated_at": "2021-06-25T19:07:33.155Z",
            })
            .to_string(),
        )
        .unwrap();

        assert_eq!(
            connection.r#type,
            KnownOrUnknown::Unknown("UnknownType".to_string())
        )
    }
}
