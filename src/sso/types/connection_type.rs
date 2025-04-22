use serde::{Deserialize, Serialize};

/// The type of a [`Connection`](crate::sso::Connection).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionType {
    /// AD FS SAML.
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/adfs-saml)
    #[serde(rename = "ADFSSAML")]
    AdFsSaml,

    /// ADP OpenID Connect (OIDC).
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/adp-oidc)
    #[serde(rename = "ADPOIDC")]
    AdpOidc,

    /// Auth0 SAML.
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/auth0-saml)
    #[serde(rename = "Auth0SAML")]
    Auth0Saml,

    /// Azure Active Directory (AD) SAML.
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/azure-ad-saml)
    #[serde(rename = "AzureSAML")]
    AzureSaml,

    /// CAS SAML.
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/cas-saml)
    #[serde(rename = "CASSAML")]
    CasSaml,

    /// ClassLink SAML.
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/classlink-saml)
    #[serde(rename = "ClassLinkSAML")]
    ClassLinkSaml,

    /// Cloudflare SAML.
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/cloudflare-saml)
    #[serde(rename = "CloudflareSAML")]
    CloudflareSaml,

    /// CyberArk SAML.
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/cyberark-saml)
    #[serde(rename = "CyberArkSAML")]
    CyberArkSaml,

    /// Duo SAML.
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/duo-saml)
    #[serde(rename = "DuoSAML")]
    DuoSaml,

    /// Generic OpenID Connect (OIDC).
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/oidc)
    #[serde(rename = "GenericOIDC")]
    GenericOidc,

    /// Generic SAML.
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/generic-saml)
    #[serde(rename = "GenericSAML")]
    GenericSaml,

    /// Google OAuth.
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/g-suite-oauth)
    #[serde(rename = "GoogleOAuth")]
    GoogleOauth,

    /// Google SAML.
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/google-saml)
    #[serde(rename = "GoogleSAML")]
    GoogleSaml,

    /// JumpCloud SAML.
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/jumpcloud-saml)
    #[serde(rename = "JumpCloudSAML")]
    JumpCloudSaml,

    /// Keycloak SAML.
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/keycloak-saml)
    #[serde(rename = "KeycloakSAML")]
    KeycloakSaml,

    /// Microsoft OAuth.
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/microsoft-oauth)
    #[serde(rename = "MicrosoftOAuth")]
    MicrosoftOauth,

    /// miniOrange SAML.
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/mini-orange-saml)
    #[serde(rename = "MiniOrangeSAML")]
    MiniOrangeSaml,

    /// NetIQ SAML.
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/net-iq-saml)
    #[serde(rename = "NetIqSAML")]
    NetIqSaml,

    /// Okta SAML.
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/okta-saml)
    #[serde(rename = "OktaSAML")]
    OktaSaml,

    /// OneLogin SAML.
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/onelogin-saml)
    #[serde(rename = "OneLoginSAML")]
    OneLoginSaml,

    /// Oracle SAML.
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/oracle-saml)
    #[serde(rename = "OracleSAML")]
    OracleSaml,

    /// PingFederate SAML.
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/ping-federate-saml)
    #[serde(rename = "PingFederateSAML")]
    PingFederateSaml,

    /// PingOne SAML.
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/ping-one-saml)
    #[serde(rename = "PingOneSAML")]
    PingOneSaml,

    /// Salesforce SAML.
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/salesforce-saml)
    #[serde(rename = "SalesforceSAML")]
    SalesforceSaml,

    /// Shibboleth SAML.
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/shibboleth)
    #[serde(rename = "ShibbolethSAML")]
    ShibbolethSaml,

    /// SimpleSAMLphp SAML.
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/simple-saml-php-saml)
    #[serde(rename = "SimpleSamlPhpSAML")]
    SimpleSamlPhpSaml,

    /// VMware SAML.
    ///
    /// [WorkOS Docs: Integration Guide](https://workos.com/docs/integrations/vmware-saml)
    #[serde(rename = "VMwareSAML")]
    VmwareSaml,
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::ConnectionType;

    #[test]
    fn it_properly_serializes_adp_oidc() {
        assert_eq!(
            serde_json::to_string(&ConnectionType::AdpOidc).unwrap(),
            json!("ADPOIDC").to_string()
        )
    }

    #[test]
    fn it_properly_deserializes_adp_oidc() {
        assert_eq!(
            serde_json::from_str::<ConnectionType>(&json!("ADPOIDC").to_string()).unwrap(),
            ConnectionType::AdpOidc
        )
    }
}
