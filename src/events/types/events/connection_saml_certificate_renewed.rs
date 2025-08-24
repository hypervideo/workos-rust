use serde::{Deserialize, Serialize};

use crate::{
    Timestamp,
    sso::{ConnectionEvent, SamlCertificateEvent},
};

/// [WorkOS Docs: `connection.saml_certificate_renewed` event](https://workos.com/docs/events/connection).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectionSamlCertificateRenewedEvent {
    /// The connection.
    pub connection: ConnectionEvent,

    /// The certificate.
    pub certificate: SamlCertificateEvent,

    /// The timestamp indicating when the certificate was renewed.
    pub renewed_at: Timestamp,
}
