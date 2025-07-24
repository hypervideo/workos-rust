use async_trait::async_trait;
use thiserror::Error;

use crate::organizations::{Organization, OrganizationId, Organizations};
use crate::{ResponseExt, WorkOsError, WorkOsResult};

/// An error returned from [`GetOrganization`].
#[derive(Debug, Error)]
pub enum GetOrganizationError {}

impl From<GetOrganizationError> for WorkOsError<GetOrganizationError> {
    fn from(err: GetOrganizationError) -> Self {
        Self::Operation(err)
    }
}

/// [WorkOS Docs: Get an Organization](https://workos.com/docs/reference/sso/organization/get)
#[async_trait]
pub trait GetOrganization {
    /// Retrieves an [`Organization`] by its ID.
    ///
    /// [WorkOS Docs: Get an Organization](https://workos.com/docs/reference/sso/organization/get)
    ///
    /// # Examples
    ///
    /// ```
    /// # use workos_sdk::WorkOsResult;
    /// # use workos_sdk::organizations::*;
    /// use workos_sdk::{ApiKey, WorkOs};
    ///
    /// # async fn run() -> WorkOsResult<(), GetOrganizationError> {
    /// let workos = WorkOs::new(&ApiKey::from("sk_example_123456789"));
    ///
    /// let organization = workos
    ///     .organizations()
    ///     .get_organization(&OrganizationId::from("org_01EHZNVPK3SFK441A1RGBFSHRT"))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn get_organization(
        &self,
        id: &OrganizationId,
    ) -> WorkOsResult<Organization, GetOrganizationError>;
}

#[async_trait]
impl GetOrganization for Organizations<'_> {
    async fn get_organization(
        &self,
        id: &OrganizationId,
    ) -> WorkOsResult<Organization, GetOrganizationError> {
        let url = self
            .workos
            .base_url()
            .join(&format!("/organizations/{id}"))?;
        let organization = self
            .workos
            .client()
            .get(url)
            .bearer_auth(self.workos.key())
            .send()
            .await?
            .handle_unauthorized_or_generic_error()?
            .json::<Organization,_>()
            .await?;

        Ok(organization)
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;
    use tokio;

    use crate::{ApiKey, WorkOs};

    use super::*;

    #[tokio::test]
    async fn it_calls_the_get_organization_endpoint() {
        let mut server = mockito::Server::new_async().await;

        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&server.url())
            .unwrap()
            .build();

        server
            .mock("GET", "/organizations/org_01EHZNVPK3SFK441A1RGBFSHRT")
            .match_header("Authorization", "Bearer sk_example_123456789")
            .with_status(200)
            .with_body(
                json!({
                  "id": "org_01EHZNVPK3SFK441A1RGBFSHRT",
                  "object": "organization",
                  "name": "Foo Corporation",
                  "allow_profiles_outside_organization": false,
                  "created_at": "2021-06-25T19:07:33.155Z",
                  "updated_at": "2021-06-25T19:07:33.155Z",
                  "domains": [
                    {
                      "domain": "foo-corp.com",
                      "id": "org_domain_01EHZNVPK2QXHMVWCEDQEKY69A",
                      "object": "organization_domain"
                    },
                    {
                      "domain": "another-foo-corp-domain.com",
                      "id": "org_domain_01EHZNS0H9W90A90FV79GAB6AB",
                      "object": "organization_domain"
                    }
                  ]
                })
                .to_string(),
            )
            .create_async()
            .await;

        let organization = workos
            .organizations()
            .get_organization(&OrganizationId::from("org_01EHZNVPK3SFK441A1RGBFSHRT"))
            .await
            .unwrap();

        assert_eq!(
            organization.id,
            OrganizationId::from("org_01EHZNVPK3SFK441A1RGBFSHRT")
        )
    }
}
