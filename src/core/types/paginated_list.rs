use serde::{Deserialize, Serialize};

/// A paginated list of records.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaginatedList<T> {
    /// The list of items in the current page.
    pub data: Vec<T>,

    /// The pagination metadata.
    #[serde(rename = "list_metadata")]
    pub metadata: ListMetadata,
}

/// The metadata for a [`PaginatedList`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListMetadata {
    /// The pagination cursor used to retrieve the previous page of records.
    pub before: Option<String>,

    /// The pagination cursor used to retrieve the next page of records.
    pub after: Option<String>,
}
