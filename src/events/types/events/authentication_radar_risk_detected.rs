use serde::{Deserialize, Serialize};

use crate::user_management::AuthenticationRadarRiskDetectedEventData;

/// [WorkOS Docs: `authentication.radar_risk_detected` event](https://workos.com/docs/events/authentication).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticationRadarRiskDetectedEvent(pub AuthenticationRadarRiskDetectedEventData);
