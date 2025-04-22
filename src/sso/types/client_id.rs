use derive_more::{Deref, Display, From};
use serde::Serialize;

/// A client ID used to initiate SSO.
///
/// Each environment will have its own client ID.
#[derive(Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[from(forward)]
pub struct ClientId(String);
