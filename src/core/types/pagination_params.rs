use serde::Serialize;

/// The parameters used to control pagination for a given paginated endpoint.
#[derive(Clone, Debug, Serialize)]
pub struct PaginationParams<'a> {
    /// The order in which records should be paginated.
    pub order: &'a PaginationOrder,

    /// The cursor after which records should be retrived.
    pub after: Option<&'a str>,

    /// The cursor before which records should be retrieved.
    pub before: Option<&'a str>,

    /// Upper limit on the number of objects to return, between 1 and 100. The default value is 10.
    pub limit: Option<u8>,
}

impl Default for PaginationParams<'_> {
    fn default() -> Self {
        Self {
            order: &PaginationOrder::DEFAULT,
            before: None,
            after: None,
            limit: None,
        }
    }
}

/// The order in which records should be returned when paginating.
#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaginationOrder {
    /// Records are returned in ascending order.
    Asc,

    /// Records are returned in descending order.
    Desc,
}

impl PaginationOrder {
    /// The default order to use for pagination.
    pub(crate) const DEFAULT: PaginationOrder = PaginationOrder::Desc;
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::PaginationOrder;

    #[test]
    fn pagination_order_properly_serializes_asc() {
        assert_eq!(
            serde_json::to_string(&PaginationOrder::Asc).unwrap(),
            json!("asc").to_string()
        )
    }

    #[test]
    fn pagination_order_properly_serializes_desc() {
        assert_eq!(
            serde_json::to_string(&PaginationOrder::Desc).unwrap(),
            json!("desc").to_string()
        )
    }
}
