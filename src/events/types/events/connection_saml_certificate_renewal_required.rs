use serde::{Deserialize, Serialize};

use crate::sso::{ConnectionEvent, SamlCertificateEvent};

/// [WorkOS Docs: `connection.saml_certificate_renewal_required` event](https://workos.com/docs/events/connection).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectionSamlCertificateRenewalRequiredEvent {
    /// The connection.
    pub connection: ConnectionEvent,

    /// The certificate.
    pub certificate: SamlCertificateEvent,

    /// The days until the certificate expires.
    pub days_until_expiry: isize,
}
