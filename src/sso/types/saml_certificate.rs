use serde::{Deserialize, Serialize};

use crate::Timestamp;

/// The state of an [`Invitation`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SamlCertificateType {
    /// The certificate type is request signing.
    RequestSigning,

    /// The certificate type is response encryption.
    ResponseEncryption,

    /// The certificate type is response signing.
    ResponseSigning,
}

/// [WorkOS Docs: Connection events](https://workos.com/docs/events/connection)
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SamlCertificateEvent {
    /// The type of the certificate.
    #[serde(rename = "certificate_type")]
    pub r#type: SamlCertificateType,

    /// The timestamp indicating when the object was created.
    pub expiry_date: Timestamp,

    /// Whether the certificated is expired.
    pub is_expired: Option<bool>,
}
