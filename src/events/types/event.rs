use std::collections::HashMap;

use derive_more::{Deref, Display, From};
use serde::{Deserialize, Serialize};

use crate::{Timestamp, events::*};

/// The ID of an [`Event`].
#[derive(
    Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[from(forward)]
pub struct EventId(String);

/// An optional object of extra information relevant to the event.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventContext(pub HashMap<String, String>);

/// The type of an [`Event`].
#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventName {
    /// [WorkOS Docs: `authentication.email_verification_failed` event](https://workos.com/docs/events/authentication).
    #[display("authentication.email_verification_failed")]
    #[serde(rename = "authentication.email_verification_failed")]
    AuthenticationEmailVerificationFailed,

    /// [WorkOS Docs: `authentication.email_verification_succeeded` event](https://workos.com/docs/events/authentication)
    #[display("authentication.email_verification_succeeded")]
    #[serde(rename = "authentication.email_verification_succeeded")]
    AuthenticationEmailVerificationSucceeded,

    /// [WorkOS Docs: `authentication.magic_auth_failed` event](https://workos.com/docs/events/authentication)
    #[display("authentication.magic_auth_failed")]
    #[serde(rename = "authentication.magic_auth_failed")]
    AuthenticationMagicAuthFailed,

    /// [WorkOS Docs: `authentication.magic_auth_succeeded` event](https://workos.com/docs/events/authentication)
    #[display("authentication.magic_auth_succeeded")]
    #[serde(rename = "authentication.magic_auth_succeeded")]
    AuthenticationMagicAuthSucceeded,

    /// [WorkOS Docs: `authentication.mfa_failed` event](https://workos.com/docs/events/authentication)
    #[display("authentication.mfa_failed")]
    #[serde(rename = "authentication.mfa_failed")]
    AuthenticationMfaFailed,

    /// [WorkOS Docs: `authentication.mfa_succeeded` event](https://workos.com/docs/events/authentication)
    #[display("authentication.mfa_succeeded")]
    #[serde(rename = "authentication.mfa_succeeded")]
    AuthenticationMfaSucceeded,

    /// [WorkOS Docs: `authentication.oauth_failed` event](https://workos.com/docs/events/authentication)
    #[display("authentication.oauth_failed")]
    #[serde(rename = "authentication.oauth_failed")]
    AuthenticationOauthFailed,

    /// [WorkOS Docs: `authentication.oauth_succeeded` event](https://workos.com/docs/events/authentication)
    #[display("authentication.oauth_succeeded")]
    #[serde(rename = "authentication.oauth_succeeded")]
    AuthenticationOauthSucceeded,

    /// [WorkOS Docs: `authentication.password_failed` event](https://workos.com/docs/events/authentication)
    #[display("authentication.password_failed")]
    #[serde(rename = "authentication.password_failed")]
    AuthenticationPasswordFailed,

    /// [WorkOS Docs: `authentication.password_succeeded` event](https://workos.com/docs/events/authentication)
    #[display("authentication.password_succeeded")]
    #[serde(rename = "authentication.password_succeeded")]
    AuthenticationPasswordSucceeded,

    /// [WorkOS Docs: `authentication.passkey_failed` event](https://workos.com/docs/events/authentication)
    #[display("authentication.passkey_failed")]
    #[serde(rename = "authentication.passkey_failed")]
    AuthenticationPasskeyFailed,

    /// [WorkOS Docs: `authentication.passkey_succeeded` event](https://workos.com/docs/events/authentication)
    #[display("authentication.passkey_succeeded")]
    #[serde(rename = "authentication.passkey_succeeded")]
    AuthenticationPasskeySucceeded,

    /// [WorkOS Docs: `authentication.sso_failed` event](https://workos.com/docs/events/authentication)
    #[display("authentication.sso_failed")]
    #[serde(rename = "authentication.sso_failed")]
    AuthenticationSsoFailed,

    /// [WorkOS Docs: `authentication.sso_succeeded` event](https://workos.com/docs/events/authentication)
    #[display("authentication.sso_succeeded")]
    #[serde(rename = "authentication.sso_succeeded")]
    AuthenticationSsoSucceeded,

    /// [WorkOS Docs: `authentication.radar_risk_detected` event](https://workos.com/docs/events/authentication)
    #[display("authentication.radar_risk_detected")]
    #[serde(rename = "authentication.radar_risk_detected")]
    AuthenticationRadarRiskDetected,

    /// [WorkOS Docs: `connection.activated` event](https://workos.com/docs/events/connection)
    #[display("connection.activated")]
    #[serde(rename = "connection.activated")]
    ConnectionActivated,

    /// [WorkOS Docs: `connection.deactivated` event](https://workos.com/docs/events/connection)
    #[display("connection.deactivated")]
    #[serde(rename = "connection.deactivated")]
    ConnectionDeactivated,

    /// [WorkOS Docs: `connection.deleted` event](https://workos.com/docs/events/connection)
    #[display("connection.deleted")]
    #[serde(rename = "connection.deleted")]
    ConnectionDeleted,

    /// [WorkOS Docs: `connection.saml_certificate_renewed` event](https://workos.com/docs/events/connection)
    #[display("connection.saml_certificate_renewed")]
    #[serde(rename = "connection.saml_certificate_renewed")]
    ConnectionSamlCertificateRenewed,

    /// [WorkOS Docs: `connection.saml_certificate_renewal_required` event](https://workos.com/docs/events/connection)
    #[display("connection.saml_certificate_renewal_required")]
    #[serde(rename = "connection.saml_certificate_renewal_required")]
    ConnectionSamlCertificateRenewalRequired,

    /// [WorkOS Docs: `dsync.activated` event](https://workos.com/docs/events/directory-sync)
    #[display("dsync.activated")]
    #[serde(rename = "dsync.activated")]
    DsyncActivated,

    /// [WorkOS Docs: `dsync.deleted` event](https://workos.com/docs/events/directory-sync)
    #[display("dsync.deleted")]
    #[serde(rename = "dsync.deleted")]
    DsyncDeleted,

    /// [WorkOS Docs: `dsync.group.created` event](https://workos.com/docs/events/directory-sync)
    #[display("dsync.group.created")]
    #[serde(rename = "dsync.group.created")]
    DsyncGroupCreated,

    /// [WorkOS Docs: `dsync.group.deleted` event](https://workos.com/docs/events/directory-sync)
    #[display("dsync.group.deleted")]
    #[serde(rename = "dsync.group.deleted")]
    DsyncGroupDeleted,

    /// [WorkOS Docs: `dsync.group.updated` event](https://workos.com/docs/events/directory-sync)
    #[display("dsync.group.updated")]
    #[serde(rename = "dsync.group.updated")]
    DsyncGroupUpdated,

    /// [WorkOS Docs: `dsync.group.user_added` event](https://workos.com/docs/events/directory-sync)
    #[display("dsync.group.user_added")]
    #[serde(rename = "dsync.group.user_added")]
    DsyncGroupUserAdded,

    /// [WorkOS Docs: `dsync.group.user_removed` event](https://workos.com/docs/events/directory-sync)
    #[display("dsync.group.user_removed")]
    #[serde(rename = "dsync.group.user_removed")]
    DsyncGroupUserRemoved,

    /// [WorkOS Docs: `dsync.user.created` event](https://workos.com/docs/events/directory-sync)
    #[display("dsync.user.created")]
    #[serde(rename = "dsync.user.created")]
    DsyncUserCreated,

    /// [WorkOS Docs: `dsync.user.deleted` event](https://workos.com/docs/events/directory-sync)
    #[display("dsync.user.deleted")]
    #[serde(rename = "dsync.user.deleted")]
    DsyncUserDeleted,

    /// [WorkOS Docs: `dsync.user.updated` event](https://workos.com/docs/events/directory-sync)
    #[display("dsync.user.updated")]
    #[serde(rename = "dsync.user.updated")]
    DsyncUserUpdated,

    /// [WorkOS Docs: `email_verification.created` event](https://workos.com/docs/events/email-verification)
    #[display("email_verification.created")]
    #[serde(rename = "email_verification.created")]
    EmailVerificationCreated,

    /// [WorkOS Docs: `invitation.accepted` event](https://workos.com/docs/events/invitation)
    #[display("invitation.accepted")]
    #[serde(rename = "invitation.accepted")]
    InvitationAccepted,

    /// [WorkOS Docs: `invitation.created` event](https://workos.com/docs/events/invitation)
    #[display("invitation.created")]
    #[serde(rename = "invitation.created")]
    InvitationCreated,

    /// [WorkOS Docs: `invitation.revoked` event](https://workos.com/docs/events/invitation)
    #[display("invitation.revoked")]
    #[serde(rename = "invitation.revoked")]
    InvitationRevoked,

    /// [WorkOS Docs: `magic_auth.created` event](https://workos.com/docs/events/magic-auth)
    #[display("magic_auth.created")]
    #[serde(rename = "magic_auth.created")]
    MagicAuthCreated,

    /// [WorkOS Docs: `organization.created` event](https://workos.com/docs/events/organization)
    #[display("organization.created")]
    #[serde(rename = "organization.created")]
    OrganizationCreated,

    /// [WorkOS Docs: `organization.updated` event](https://workos.com/docs/events/organization)
    #[display("organization.updated")]
    #[serde(rename = "organization.updated")]
    OrganizationUpdated,

    /// [WorkOS Docs: `organization.deleted` event](https://workos.com/docs/events/organization)
    #[display("organization.deleted")]
    #[serde(rename = "organization.deleted")]
    OrganizationDeleted,

    /// [WorkOS Docs: `organization_domain.created` event](https://workos.com/docs/events/organization-domain)
    #[display("organization_domain.created")]
    #[serde(rename = "organization_domain.created")]
    OrganizationDomainCreated,

    /// [WorkOS Docs: `organization_domain.updated` event](https://workos.com/docs/events/organization-domain)
    #[display("organization_domain.updated")]
    #[serde(rename = "organization_domain.updated")]
    OrganizationDomainUpdated,

    /// [WorkOS Docs: `organization_domain.deleted` event](https://workos.com/docs/events/organization-domain)
    #[display("organization_domain.deleted")]
    #[serde(rename = "organization_domain.deleted")]
    OrganizationDomainDeleted,

    /// [WorkOS Docs: `organization_domain.verified` event](https://workos.com/docs/events/organization-domain)
    #[display("organization_domain.verified")]
    #[serde(rename = "organization_domain.verified")]
    OrganizationDomainVerified,

    /// [WorkOS Docs: `organization_domain.verification_failed` event](https://workos.com/docs/events/organization-domain)
    #[display("organization_domain.verification_failed")]
    #[serde(rename = "organization_domain.verification_failed")]
    OrganizationDomainVerificationFailed,

    /// [WorkOS Docs: `organization_membership.created` event](https://workos.com/docs/events/organization-membership)
    #[display("organization_membership.created")]
    #[serde(rename = "organization_membership.created")]
    OrganizationMembershipCreated,

    /// [WorkOS Docs: `organization_membership.deleted` event](https://workos.com/docs/events/organization-membership)
    #[display("organization_membership.deleted")]
    #[serde(rename = "organization_membership.deleted")]
    OrganizationMembershipDeleted,

    /// [WorkOS Docs: `organization_membership.updated` event](https://workos.com/docs/events/organization-membership)
    #[display("organization_membership.updated")]
    #[serde(rename = "organization_membership.updated")]
    OrganizationMembershipUpdated,

    /// [WorkOS Docs: `password_reset.created` event](https://workos.com/docs/events/password-reset)
    #[display("password_reset.created")]
    #[serde(rename = "password_reset.created")]
    PasswordResetCreated,

    /// [WorkOS Docs: `password_reset.succeeded` event](https://workos.com/docs/events/password-reset)
    #[display("password_reset.succeeded")]
    #[serde(rename = "password_reset.succeeded")]
    PasswordResetSucceeded,

    /// [WorkOS Docs: `role.created` event](https://workos.com/docs/events/role)
    #[display("role.created")]
    #[serde(rename = "role.created")]
    RoleCreated,

    /// [WorkOS Docs: `role.deleted` event](https://workos.com/docs/events/role)
    #[display("role.deleted")]
    #[serde(rename = "role.deleted")]
    RoleDeleted,

    /// [WorkOS Docs: `role.updated` event](https://workos.com/docs/events/role)
    #[display("role.updated")]
    #[serde(rename = "role.updated")]
    RoleUpdated,

    /// [WorkOS Docs: `session.created` event](https://workos.com/docs/events/session)
    #[display("session.created")]
    #[serde(rename = "session.created")]
    SessionCreated,

    /// [WorkOS Docs: `session.revoked` event](https://workos.com/docs/events/session)
    #[display("session.revoked")]
    #[serde(rename = "session.revoked")]
    SessionRevoked,

    /// [WorkOS Docs: `user.created` event](https://workos.com/docs/events/user)
    #[display("user.created")]
    #[serde(rename = "user.created")]
    UserCreated,

    /// [WorkOS Docs: `user.deleted` event](https://workos.com/docs/events/user)
    #[display("user.deleted")]
    #[serde(rename = "user.deleted")]
    UserDeleted,

    /// [WorkOS Docs: `user.updated` event](https://workos.com/docs/events/user)
    #[display("user.updated")]
    #[serde(rename = "user.updated")]
    UserUpdated,
}

/// The data of the [`Event`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "event", content = "data")]
pub enum EventData {
    /// [WorkOS Docs: `authentication.email_verification_failed` event](https://workos.com/docs/events/authentication).
    #[serde(rename = "authentication.email_verification_failed")]
    AuthenticationEmailVerificationFailed(AuthenticationEmailVerificationFailedEvent),

    /// [WorkOS Docs: `authentication.email_verification_succeeded` event](https://workos.com/docs/events/authentication)
    #[serde(rename = "authentication.email_verification_succeeded")]
    AuthenticationEmailVerificationSucceeded(AuthenticationEmailVerificationSucceededEvent),

    /// [WorkOS Docs: `authentication.magic_auth_failed` event](https://workos.com/docs/events/authentication)
    #[serde(rename = "authentication.magic_auth_failed")]
    AuthenticationMagicAuthFailed(AuthenticationMagicAuthFailedEvent),

    /// [WorkOS Docs: `authentication.magic_auth_succeeded` event](https://workos.com/docs/events/authentication)
    #[serde(rename = "authentication.magic_auth_succeeded")]
    AuthenticationMagicAuthSucceeded(AuthenticationMagicAuthSucceededEvent),

    /// [WorkOS Docs: `authentication.mfa_failed` event](https://workos.com/docs/events/authentication)
    #[serde(rename = "authentication.mfa_failed")]
    AuthenticationMfaFailed(AuthenticationMfaFailedEvent),

    /// [WorkOS Docs: `authentication.mfa_succeeded` event](https://workos.com/docs/events/authentication)
    #[serde(rename = "authentication.mfa_succeeded")]
    AuthenticationMfaSucceeded(AuthenticationMfaSucceededEvent),

    /// [WorkOS Docs: `authentication.oauth_failed` event](https://workos.com/docs/events/authentication)
    #[serde(rename = "authentication.oauth_failed")]
    AuthenticationOauthFailed(AuthenticationOauthFailedEvent),

    /// [WorkOS Docs: `authentication.oauth_succeeded` event](https://workos.com/docs/events/authentication)
    #[serde(rename = "authentication.oauth_succeeded")]
    AuthenticationOauthSucceeded(AuthenticationOauthSucceededEvent),

    /// [WorkOS Docs: `authentication.password_failed` event](https://workos.com/docs/events/authentication)
    #[serde(rename = "authentication.password_failed")]
    AuthenticationPasswordFailed(AuthenticationPasswordFailedEvent),

    /// [WorkOS Docs: `authentication.password_succeeded` event](https://workos.com/docs/events/authentication)
    #[serde(rename = "authentication.password_succeeded")]
    AuthenticationPasswordSucceeded(AuthenticationPasswordSucceededEvent),

    /// [WorkOS Docs: `authentication.passkey_failed` event](https://workos.com/docs/events/authentication)
    #[serde(rename = "authentication.passkey_failed")]
    AuthenticationPasskeyFailed(AuthenticationPasskeyFailedEvent),

    /// [WorkOS Docs: `authentication.passkey_succeeded` event](https://workos.com/docs/events/authentication)
    #[serde(rename = "authentication.passkey_succeeded")]
    AuthenticationPasskeySucceeded(AuthenticationPasskeySucceededEvent),

    /// [WorkOS Docs: `authentication.sso_failed` event](https://workos.com/docs/events/authentication)
    #[serde(rename = "authentication.sso_failed")]
    AuthenticationSsoFailed(AuthenticationSsoFailedEvent),

    /// [WorkOS Docs: `authentication.sso_succeeded` event](https://workos.com/docs/events/authentication)
    #[serde(rename = "authentication.sso_succeeded")]
    AuthenticationSsoSucceeded(AuthenticationSsoSucceededEvent),

    /// [WorkOS Docs: `authentication.radar_risk_detected` event](https://workos.com/docs/events/authentication)
    #[serde(rename = "authentication.radar_risk_detected")]
    AuthenticationRadarRiskDetected(AuthenticationRadarRiskDetectedEvent),

    /// [WorkOS Docs: `connection.activated` event](https://workos.com/docs/events/connection)
    #[serde(rename = "connection.activated")]
    ConnectionActivated(ConnectionActivatedEvent),

    /// [WorkOS Docs: `connection.deactivated` event](https://workos.com/docs/events/connection)
    #[serde(rename = "connection.deactivated")]
    ConnectionDeactivated(ConnectionDeactivatedEvent),

    /// [WorkOS Docs: `connection.deleted` event](https://workos.com/docs/events/connection)
    #[serde(rename = "connection.deleted")]
    ConnectionDeleted(ConnectionDeletedEvent),

    /// [WorkOS Docs: `connection.saml_certificate_renewed` event](https://workos.com/docs/events/connection)
    #[serde(rename = "connection.saml_certificate_renewed")]
    ConnectionSamlCertificateRenewed(ConnectionSamlCertificateRenewedEvent),

    /// [WorkOS Docs: `connection.saml_certificate_renewal_required` event](https://workos.com/docs/events/connection)
    #[serde(rename = "connection.saml_certificate_renewal_required")]
    ConnectionSamlCertificateRenewalRequired(ConnectionSamlCertificateRenewalRequiredEvent),

    /// [WorkOS Docs: `dsync.activated` event](https://workos.com/docs/events/directory-sync)
    #[serde(rename = "dsync.activated")]
    DsyncActivated(DsyncActivatedEvent),

    /// [WorkOS Docs: `dsync.deleted` event](https://workos.com/docs/events/directory-sync)
    #[serde(rename = "dsync.deleted")]
    DsyncDeleted(DsyncDeletedEvent),

    /// [WorkOS Docs: `dsync.group.created` event](https://workos.com/docs/events/directory-sync)
    #[serde(rename = "dsync.group.created")]
    DsyncGroupCreated(DsyncGroupCreatedEvent),

    /// [WorkOS Docs: `dsync.group.deleted` event](https://workos.com/docs/events/directory-sync)
    #[serde(rename = "dsync.group.deleted")]
    DsyncGroupDeleted(DsyncGroupDeletedEvent),

    /// [WorkOS Docs: `dsync.group.updated` event](https://workos.com/docs/events/directory-sync)
    #[serde(rename = "dsync.group.updated")]
    DsyncGroupUpdated(DsyncGroupUpdatedEvent),

    /// [WorkOS Docs: `dsync.group.user_added` event](https://workos.com/docs/events/directory-sync)
    #[serde(rename = "dsync.group.user_added")]
    DsyncGroupUserAdded(DsyncGroupUserAddedEvent),

    /// [WorkOS Docs: `dsync.group.user_removed` event](https://workos.com/docs/events/directory-sync)
    #[serde(rename = "dsync.group.user_removed")]
    DsyncGroupUserRemoved(DsyncGroupUserRemovedEvent),

    /// [WorkOS Docs: `dsync.user.created` event](https://workos.com/docs/events/directory-sync)
    #[serde(rename = "dsync.user.created")]
    DsyncUserCreated(DsyncUserCreatedEvent),

    /// [WorkOS Docs: `dsync.user.deleted` event](https://workos.com/docs/events/directory-sync)
    #[serde(rename = "dsync.user.deleted")]
    DsyncUserDeleted(DsyncUserDeletedEvent),

    /// [WorkOS Docs: `dsync.user.updated` event](https://workos.com/docs/events/directory-sync)
    #[serde(rename = "dsync.user.updated")]
    DsyncUserUpdated(DsyncUserUpdatedEvent),

    /// [WorkOS Docs: `email_verification.created` event](https://workos.com/docs/events/email-verification)
    #[serde(rename = "email_verification.created")]
    EmailVerificationCreated(EmailVerificationCreatedEvent),

    /// [WorkOS Docs: `invitation.accepted` event](https://workos.com/docs/events/invitation)
    #[serde(rename = "invitation.accepted")]
    InvitationAccepted(InvitationAcceptedEvent),

    /// [WorkOS Docs: `invitation.created` event](https://workos.com/docs/events/invitation)
    #[serde(rename = "invitation.created")]
    InvitationCreated(InvitationCreatedEvent),

    /// [WorkOS Docs: `invitation.revoked` event](https://workos.com/docs/events/invitation)
    #[serde(rename = "invitation.revoked")]
    InvitationRevoked(InvitationRevokedEvent),

    /// [WorkOS Docs: `magic_auth.created` event](https://workos.com/docs/events/magic-auth)
    #[serde(rename = "magic_auth.created")]
    MagicAuthCreated(MagicAuthCreatedEvent),

    /// [WorkOS Docs: `organization.created` event](https://workos.com/docs/events/organization)
    #[serde(rename = "organization.created")]
    OrganizationCreated(OrganizationCreatedEvent),

    /// [WorkOS Docs: `organization.updated` event](https://workos.com/docs/events/organization)
    #[serde(rename = "organization.updated")]
    OrganizationUpdated(OrganizationUpdatedEvent),

    /// [WorkOS Docs: `organization.deleted` event](https://workos.com/docs/events/organization)
    #[serde(rename = "organization.deleted")]
    OrganizationDeleted(OrganizationDeletedEvent),

    /// [WorkOS Docs: `organization_domain.created` event](https://workos.com/docs/events/organization-domain)
    #[serde(rename = "organization_domain.created")]
    OrganizationDomainCreated(OrganizationDomainCreatedEvent),

    /// [WorkOS Docs: `organization_domain.updated` event](https://workos.com/docs/events/organization-domain)
    #[serde(rename = "organization_domain.updated")]
    OrganizationDomainUpdated(OrganizationDomainUpdatedEvent),

    /// [WorkOS Docs: `organization_domain.deleted` event](https://workos.com/docs/events/organization-domain)
    #[serde(rename = "organization_domain.deleted")]
    OrganizationDomainDeleted(OrganizationDomainDeletedEvent),

    /// [WorkOS Docs: `organization_domain.verified` event](https://workos.com/docs/events/organization-domain)
    #[serde(rename = "organization_domain.verified")]
    OrganizationDomainVerified(OrganizationDomainVerifiedEvent),

    /// [WorkOS Docs: `organization_domain.verification_failed` event](https://workos.com/docs/events/organization-domain)
    #[serde(rename = "organization_domain.verification_failed")]
    OrganizationDomainVerificationFailed(OrganizationDomainVerificationFailedEvent),

    /// [WorkOS Docs: `organization_membership.created` event](https://workos.com/docs/events/organization-membership)
    #[serde(rename = "organization_membership.created")]
    OrganizationMembershipCreated(OrganizationMembershipCreatedEvent),

    /// [WorkOS Docs: `organization_membership.deleted` event](https://workos.com/docs/events/organization-membership)
    #[serde(rename = "organization_membership.deleted")]
    OrganizationMembershipDeleted(OrganizationMembershipDeletedEvent),

    /// [WorkOS Docs: `organization_membership.updated` event](https://workos.com/docs/events/organization-membership)
    #[serde(rename = "organization_membership.updated")]
    OrganizationMembershipUpdated(OrganizationMembershipUpdatedEvent),

    /// [WorkOS Docs: `password_reset.created` event](https://workos.com/docs/events/password-reset)
    #[serde(rename = "password_reset.created")]
    PasswordResetCreated(PasswordResetCreatedEvent),

    /// [WorkOS Docs: `password_reset.succeeded` event](https://workos.com/docs/events/password-reset)
    #[serde(rename = "password_reset.succeeded")]
    PasswordResetSucceeded(PasswordResetSucceededEvent),

    /// [WorkOS Docs: `role.created` event](https://workos.com/docs/events/role)
    #[serde(rename = "role.created")]
    RoleCreated(RoleCreatedEvent),

    /// [WorkOS Docs: `role.deleted` event](https://workos.com/docs/events/role)
    #[serde(rename = "role.deleted")]
    RoleDeleted(RoleDeletedEvent),

    /// [WorkOS Docs: `role.updated` event](https://workos.com/docs/events/role)
    #[serde(rename = "role.updated")]
    RoleUpdated(RoleUpdatedEvent),

    /// [WorkOS Docs: `session.created` event](https://workos.com/docs/events/session)
    #[serde(rename = "session.created")]
    SessionCreated(SessionCreatedEvent),

    /// [WorkOS Docs: `session.revoked` event](https://workos.com/docs/events/session)
    #[serde(rename = "session.revoked")]
    SessionRevoked(SessionRevokedEvent),

    /// [WorkOS Docs: `user.created` event](https://workos.com/docs/events/user)
    #[serde(rename = "user.created")]
    UserCreated(UserCreatedEvent),

    /// [WorkOS Docs: `user.deleted` event](https://workos.com/docs/events/user)
    #[serde(rename = "user.deleted")]
    UserDeleted(UserDeletedEvent),

    /// [WorkOS Docs: `user.updated` event](https://workos.com/docs/events/user)
    #[serde(rename = "user.updated")]
    UserUpdated(UserUpdatedEvent),
}

/// [WorkOS Docs: Event](https://workos.com/docs/reference/event)
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    /// Unique identifier for the event.
    pub id: EventId,

    /// Event data.
    #[serde(flatten)]
    pub data: EventData,

    /// Timestamp of when the event occurred.
    pub created_at: Timestamp,

    /// An optional object of extra information relevant to the event.
    pub context: Option<EventContext>,
}
