use async_trait::async_trait;
use thiserror::Error;

use crate::user_management::{OrganizationMembershipId, UserManagement};
use crate::{ResponseExt, WorkOsError, WorkOsResult};

/// An error returned from [`DeleteOrganizationMembership`].
#[derive(Debug, Error)]
pub enum DeleteOrganizationMembershipError {}

impl From<DeleteOrganizationMembershipError> for WorkOsError<DeleteOrganizationMembershipError> {
    fn from(err: DeleteOrganizationMembershipError) -> Self {
        Self::Operation(err)
    }
}

/// [WorkOS Docs: Delete Organization Membership](https://workos.com/docs/reference/authkit/organization-membership#delete-organization-membership)
#[async_trait]
pub trait DeleteOrganizationMembership {
    /// Deletes an [`OrganizationMembership`].
    ///
    /// [WorkOS Docs: Delete Organization Membership](https://workos.com/docs/reference/authkit/organization-membership#delete-organization-membership)
    ///
    /// # Examples
    ///
    /// ```
    /// # use workos_sdk::WorkOsResult;
    /// # use workos_sdk::user_management::*;
    /// use workos_sdk::{ApiKey, WorkOs};
    ///
    /// # async fn run() -> WorkOsResult<(), DeleteOrganizationMembershipError> {
    /// let workos = WorkOs::new(&ApiKey::from("sk_example_123456789"));
    ///
    /// workos
    ///     .user_management()
    ///     .delete_organization_membership(&OrganizationMembershipId::from("om_01E4ZCR3C56J083X43JQXF3JK5"))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn delete_organization_membership(
        &self,
        organization_membership_id: &OrganizationMembershipId,
    ) -> WorkOsResult<(), DeleteOrganizationMembershipError>;
}

#[async_trait]
impl DeleteOrganizationMembership for UserManagement<'_> {
    async fn delete_organization_membership(
        &self,
        organization_membership_id: &OrganizationMembershipId,
    ) -> WorkOsResult<(), DeleteOrganizationMembershipError> {
        let url = self
            .workos
            .base_url()
            .join(&format!("/user_management/organization_memberships/{}", organization_membership_id))?;
        
        self
            .workos
            .client()
            .delete(url)
            .bearer_auth(self.workos.key())
            .send()
            .await?
            .handle_unauthorized_or_generic_error().await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use tokio;

    use crate::{ApiKey, WorkOs};

    use super::*;

    #[tokio::test]
    async fn it_calls_the_delete_organization_membership_endpoint() {
        let mut server = mockito::Server::new_async().await;

        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&server.url())
            .unwrap()
            .build();

        server
            .mock("DELETE", "/user_management/organization_memberships/om_01E4ZCR3C56J083X43JQXF3JK5")
            .match_header("Authorization", "Bearer sk_example_123456789")
            .with_status(204)
            .create_async()
            .await;

        workos
            .user_management()
            .delete_organization_membership(&OrganizationMembershipId::from("om_01E4ZCR3C56J083X43JQXF3JK5"))
            .await
            .unwrap();
    }
}