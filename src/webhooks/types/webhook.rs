use derive_more::{Deref, Display, From};
use serde::{Deserialize, Serialize};

use crate::webhooks::WebhookEvent;

/// The ID of a [`Webhook`].
#[derive(
    Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[from(forward)]
pub struct WebhookId(String);

/// A WorkOS webhook.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct Webhook {
    /// The ID of the webhook.
    pub id: WebhookId,

    /// The webhook event.
    #[serde(flatten)]
    pub event: WebhookEvent,
}
