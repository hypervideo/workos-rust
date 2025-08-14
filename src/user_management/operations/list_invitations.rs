use async_trait::async_trait;
use serde::Serialize;
use thiserror::Error;

use crate::user_management::{Invitation,  UserManagement};
use crate::{PaginatedList, PaginationParams, ResponseExt, WorkOsError, WorkOsResult};
use crate::organizations::OrganizationId;

/// The parameters for [`ListInvitations`].
#[derive(Debug, Serialize, Default)]
pub struct ListInvitationParams<'a> {
    /// The email address of the recipient.
    pub email: &'a str,

    /// The ID of the organization that the recipient will join.
    pub organization_id: Option<&'a OrganizationId>,

    /// The pagination parameters to use when listing users.
    #[serde(flatten)]
    pub pagination: PaginationParams<'a>,
}

/// An error returned from [`ListInvitations`].
#[derive(Debug, Error)]
pub enum ListInvitationsError {}

impl From<ListInvitationsError> for WorkOsError<ListInvitationsError> {
    fn from(err: ListInvitationsError) -> Self {
        Self::Operation(err)
    }
}

/// [WorkOS Docs: List invitations](https://workos.com/docs/reference/user-management/invitation/list)
#[async_trait]
pub trait ListInvitations {
    /// Get a list of all the invitations matching the criteria specified.
    ///
    /// [WorkOS Docs: List invitations](https://workos.com/docs/reference/user-management/invitation/list)
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    ///
    /// # use workos_sdk::WorkOsResult;
    /// # use workos_sdk::user_management::*;
    /// use workos_sdk::{ApiKey, WorkOs};
    /// #
    /// use workos_sdk::organizations::OrganizationId;
    ///
    /// async fn run() -> WorkOsResult<(), ListInvitationsError> {
    /// let workos = WorkOs::new(&ApiKey::from("sk_example_123456789"));
    ///
    /// let invitations = workos
    ///     .user_management()
    ///     .list_invitations(&ListInvitationParams {
    ///         email: "marcelina.davis@example.com",
    ///         organization_id: Some(&OrganizationId::from("org_01E4ZCR3C56J083X43JQXF3JK5")),
    ///         ..Default::default()
    ///     })
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn list_invitations(
        &self,
        params: &ListInvitationParams,
    ) -> WorkOsResult<PaginatedList<Invitation>, ListInvitationsError>;
}

#[async_trait]
impl ListInvitations for UserManagement<'_> {
    async fn list_invitations(
        &self,
        params: &ListInvitationParams,
    ) -> WorkOsResult<PaginatedList<Invitation>, ListInvitationsError> {
        let url = self
            .workos
            .base_url()
            .join("user_management/invitations")?;

        let invitations = self
            .workos
            .client()
            .get(url)
            .bearer_auth(self.workos.key())
            .query(&params)
            .send()
            .await?
            .handle_unauthorized_or_generic_error()?
            .json::<PaginatedList<Invitation>>()
            .await?;

        Ok(invitations)
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;
    use tokio;

    use crate::{ApiKey, WorkOs};
    use crate::user_management::{InvitationId, UserId};
    use super::*;

    #[tokio::test]
    async fn it_calls_the_get_list_invitations_endpoint() {
        let mut server = mockito::Server::new_async().await;

        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&server.url())
            .unwrap()
            .build();

        server
            .mock(
                "GET",
                "/user_management/invitations/invitation_123456789",
            )
            .match_header("Authorization", "Bearer sk_example_123456789")
            .with_status(200)
            .with_body(
                json!({
                  "data": [
                    {
                      "object": "invitation",
                      "id": "invitation_01E4ZCR3C56J083X43JQXF3JK5",
                      "email": "marcelina.davis@example.com",
                      "state": "pending",
                      "accepted_at": null,
                      "revoked_at": null,
                      "expires_at": "2021-07-01T19:07:33.155Z",
                      "token": "Z1uX3RbwcIl5fIGJJJCXXisdI",
                      "accept_invitation_url": "https://your-app.com/invite?invitation_token=Z1uX3RbwcIl5fIGJJJCXXisdI",
                      "organization_id": "org_01E4ZCR3C56J083X43JQXF3JK5",
                      "inviter_user_id": "user_01HYGBX8ZGD19949T3BM4FW1C3",
                      "created_at": "2021-06-25T19:07:33.155Z",
                      "updated_at": "2021-06-25T19:07:33.155Z"
                    }
                  ],
                  "list_metadata": {
                    "before": "invitation_01E4ZCR3C56J083X43JQXF3JK5",
                    "after": "invitation_01EJBGJT2PC6638TN5Y380M40Z"
                  }
                })
                .to_string(),
            )
            .create_async()
            .await;

        let paginated_invitations = workos
            .user_management()
            .list_invitations(&ListInvitationParams {
                email: "marcelina.davis@example.com",
                organization_id: Some(&OrganizationId::from("org_01E4ZCR3C56J083X43JQXF3JK5")),
                ..Default::default()
            })
            .await
            .unwrap();

        assert_eq!(
            paginated_invitations.data.into_iter().next().map(|invitation| invitation.id),
            Some(InvitationId::from("invitation_01E4ZCR3C56J083X43JQXF3JK5"))
        )
    }
}
