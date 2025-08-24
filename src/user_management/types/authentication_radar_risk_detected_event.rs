use std::net::IpAddr;

use serde::{Deserialize, Serialize};

use crate::{KnownOrUnknown, user_management::UserId};

/// The action of a [`AuthenticationRadarRiskDetectedEvent`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthenticationRadarRiskDetectedEventAction {
    /// The radar risk event is related to sign-up.
    Signup,

    /// The radar risk event is related to login.
    Login,
}

/// [WorkOS Docs: Authentication events](https://workos.com/docs/events/authentication)
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticationRadarRiskDetectedEventData {
    /// The authentication method of the radar risk event.
    pub auth_method: String,

    /// The action of the radar risk event.
    pub action: KnownOrUnknown<AuthenticationRadarRiskDetectedEventAction, String>,

    /// The blocklist type of the radar risk event.
    pub blocklist_type: String,

    /// The IP address of the radar risk event.
    pub ip_address: Option<IpAddr>,

    /// The user agent of the radar risk event.
    pub user_agent: Option<String>,

    /// The user ID of the radar risk event.
    pub user_id: UserId,

    /// The email of the radar risk event.
    pub email: String,
}
