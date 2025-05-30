use std::collections::HashMap;

use derive_more::{Deref, Display, From};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::directory_sync::DirectoryId;
use crate::organizations::OrganizationId;
use crate::{KnownOrUnknown, RawAttributes, Timestamps};

/// The ID of a [`DirectoryUser`].
#[derive(
    Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[from(forward)]
pub struct DirectoryUserId(String);

/// [WorkOS Docs: Directory User](https://workos.com/docs/reference/directory-sync/directory-user)
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectoryUser<TCustomAttributes = HashMap<String, Value>> {
    /// The ID of the directory user.
    pub id: DirectoryUserId,

    /// The unique identifier for the directory user, assigned by the Directory Provider.
    /// Different Directory Providers use different ID formats.
    pub idp_id: String,

    /// The identifier of the [`Directory`](crate::directory_sync::Directory) the directory user belongs to.
    pub directory_id: DirectoryId,

    /// The ID of the organization in which the directory resides.
    pub organization_id: Option<OrganizationId>,

    /// The username of the directory user.
    pub username: Option<String>,

    /// The emails of the directory user.
    pub emails: Vec<DirectoryUserEmail>,

    /// The name of the directory user.
    pub first_name: Option<String>,

    /// The last name of the directory user.
    pub last_name: Option<String>,

    /// The state of the directory user.
    pub state: KnownOrUnknown<DirectoryUserState, String>,

    /// The custom attributes mapped from the Directory Provider.
    pub custom_attributes: TCustomAttributes,

    /// The raw attributes received from the Directory Provider.
    pub raw_attributes: RawAttributes,

    /// The timestamps for the directory user.
    #[serde(flatten)]
    pub timestamps: Timestamps,
}

impl DirectoryUser {
    /// Returns the first primary email for the [`DirectoryUser`].
    ///
    /// Returns [`None`] if the directory user does not have a primary email.
    pub fn primary_email(&self) -> Option<&DirectoryUserEmail> {
        self.emails.iter().find(|email| email.primary == Some(true))
    }
}

/// The state of a [`DirectoryUser`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DirectoryUserState {
    /// The directory user is active.
    Active,

    /// The directory user is inactive.
    Inactive,

    /// The directory user was suspended from the directory.
    Suspended,
}

/// An email address for a [`DirectoryUser`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectoryUserEmail {
    /// Whether this is the directory user's primary email address.
    pub primary: Option<bool>,

    /// The type of the email address.
    pub r#type: Option<String>,

    /// The email address.
    pub value: Option<String>,
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use serde::Deserialize;
    use serde_json::{Value, json};

    use crate::organizations::OrganizationId;
    use crate::{KnownOrUnknown, RawAttributes, Timestamp, Timestamps};

    use super::{
        DirectoryId, DirectoryUser, DirectoryUserEmail, DirectoryUserId, DirectoryUserState,
    };

    #[test]
    fn it_deserializes_a_directory_user() {
        let directory_user: DirectoryUser = serde_json::from_str(
            &json!({
                "id": "directory_user_01E1JG7J09H96KYP8HM9B0G5SJ",
                "idp_id": "2836",
                "directory_id": "directory_01ECAZ4NV9QMV47GW873HDCX74",
                "organization_id": "org_01EZTR6WYX1A0DSE2CYMGXQ24Y",
                "first_name": "Marcelina",
                "last_name": "Davis",
                "emails": [
                    {
                        "primary": true,
                        "type": "work",
                        "value": "marcelina@foo-corp.com"
                    }
                ],
                "username": "marcelina@foo-corp.com",
                "groups": [
                    {
                        "id": "directory_group_01E64QTDNS0EGJ0FMCVY9BWGZT",
                        "name": "Engineering",
                        "created_at": "2021-06-25T19:07:33.155Z",
                        "updated_at": "2021-06-25T19:07:33.155Z",
                        "raw_attributes": {}
                    }
                ],
                "state": "active",
                "created_at": "2021-06-25T19:07:33.155Z",
                "updated_at": "2021-06-25T19:07:33.155Z",
                "custom_attributes": {
                    "department": "Engineering"
                },
                "raw_attributes": {
                    "idp_id": "2836"
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
        expected_raw_attributes.insert("idp_id".to_string(), Value::String("2836".to_string()));

        assert_eq!(
            directory_user,
            DirectoryUser {
                id: DirectoryUserId::from("directory_user_01E1JG7J09H96KYP8HM9B0G5SJ"),
                idp_id: "2836".to_string(),
                directory_id: DirectoryId::from("directory_01ECAZ4NV9QMV47GW873HDCX74"),
                organization_id: Some(OrganizationId::from("org_01EZTR6WYX1A0DSE2CYMGXQ24Y")),
                username: Some("marcelina@foo-corp.com".to_string()),
                emails: vec![DirectoryUserEmail {
                    primary: Some(true),
                    r#type: Some("work".to_string()),
                    value: Some("marcelina@foo-corp.com".to_string())
                }],
                first_name: Some("Marcelina".to_string()),
                last_name: Some("Davis".to_string()),
                state: KnownOrUnknown::Known(DirectoryUserState::Active),
                custom_attributes: expected_custom_attributes,
                raw_attributes: RawAttributes(expected_raw_attributes),
                timestamps: Timestamps {
                    created_at: Timestamp::try_from("2021-06-25T19:07:33.155Z").unwrap(),
                    updated_at: Timestamp::try_from("2021-06-25T19:07:33.155Z").unwrap(),
                }
            }
        )
    }

    #[test]
    fn it_deserializes_a_directory_user_with_a_provided_custom_attributes_type() {
        #[derive(Debug, PartialEq, Eq, Deserialize)]
        struct MyCustomAttributes {
            pub department: String,
        }

        let directory_user: DirectoryUser<MyCustomAttributes> = serde_json::from_str(
            &json!({
                "id": "directory_user_01E1JG7J09H96KYP8HM9B0G5SJ",
                "idp_id": "2836",
                "directory_id": "directory_01ECAZ4NV9QMV47GW873HDCX74",
                "first_name": "Marcelina",
                "last_name": "Davis",
                "emails": [
                    {
                        "primary": true,
                        "type": "work",
                        "value": "marcelina@foo-corp.com"
                    }
                ],
                "username": "marcelina@foo-corp.com",
                "groups": [
                    {
                        "id": "directory_group_01E64QTDNS0EGJ0FMCVY9BWGZT",
                        "name": "Engineering",
                        "created_at": "2021-06-25T19:07:33.155Z",
                        "updated_at": "2021-06-25T19:07:33.155Z",
                        "raw_attributes": {}
                    }
                ],
                "state": "active",
                "created_at": "2021-06-25T19:07:33.155Z",
                "updated_at": "2021-06-25T19:07:33.155Z",
                "custom_attributes": {
                    "department": "Engineering"
                },
                "raw_attributes": {
                    "idp_id": "2836"
                }
            })
            .to_string(),
        )
        .unwrap();

        assert_eq!(
            directory_user.custom_attributes,
            MyCustomAttributes {
                department: "Engineering".to_string()
            }
        )
    }

    #[test]
    fn it_returns_the_primary_email_when_the_user_has_a_primary_email() {
        let directory_user = DirectoryUser {
            id: DirectoryUserId::from("directory_user_01E1JG7J09H96KYP8HM9B0G5SJ"),
            idp_id: "2836".to_string(),
            directory_id: DirectoryId::from("directory_01ECAZ4NV9QMV47GW873HDCX74"),
            organization_id: Some(OrganizationId::from("org_01EZTR6WYX1A0DSE2CYMGXQ24Y")),
            username: Some("marcelina@foo-corp.com".to_string()),
            emails: vec![DirectoryUserEmail {
                primary: Some(true),
                r#type: Some("work".to_string()),
                value: Some("marcelina@foo-corp.com".to_string()),
            }],
            first_name: Some("Marcelina".to_string()),
            last_name: Some("Davis".to_string()),
            state: KnownOrUnknown::Known(DirectoryUserState::Active),
            custom_attributes: HashMap::new(),
            raw_attributes: RawAttributes(HashMap::new()),
            timestamps: Timestamps {
                created_at: Timestamp::try_from("2021-06-25T19:07:33.155Z").unwrap(),
                updated_at: Timestamp::try_from("2021-06-25T19:07:33.155Z").unwrap(),
            },
        };

        let primary_email = directory_user.primary_email();

        assert_eq!(
            primary_email,
            Some(&DirectoryUserEmail {
                primary: Some(true),
                r#type: Some("work".to_string()),
                value: Some("marcelina@foo-corp.com".to_string())
            })
        )
    }

    #[test]
    fn it_returns_none_for_the_primary_email_when_the_user_does_not_have_a_primary_email() {
        let directory_user = DirectoryUser {
            id: DirectoryUserId::from("directory_user_01E1JG7J09H96KYP8HM9B0G5SJ"),
            idp_id: "2836".to_string(),
            directory_id: DirectoryId::from("directory_01ECAZ4NV9QMV47GW873HDCX74"),
            organization_id: Some(OrganizationId::from("org_01EZTR6WYX1A0DSE2CYMGXQ24Y")),
            username: Some("marcelina@foo-corp.com".to_string()),
            emails: vec![DirectoryUserEmail {
                primary: Some(false),
                r#type: Some("work".to_string()),
                value: Some("marcelina@foo-corp.com".to_string()),
            }],
            first_name: Some("Marcelina".to_string()),
            last_name: Some("Davis".to_string()),
            state: KnownOrUnknown::Known(DirectoryUserState::Active),
            custom_attributes: HashMap::new(),
            raw_attributes: RawAttributes(HashMap::new()),
            timestamps: Timestamps {
                created_at: Timestamp::try_from("2021-06-25T19:07:33.155Z").unwrap(),
                updated_at: Timestamp::try_from("2021-06-25T19:07:33.155Z").unwrap(),
            },
        };

        let primary_email = directory_user.primary_email();

        assert_eq!(primary_email, None)
    }
}
