use derive_more::{Deref, Display, From};
use serde::Serialize;

/// The ID of a session.
#[derive(Clone, Debug, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[from(forward)]
pub struct SessionId(String);
