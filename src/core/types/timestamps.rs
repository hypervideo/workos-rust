use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

/// A UTC timestamp.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Timestamp(pub DateTime<FixedOffset>);

impl TryFrom<String> for Timestamp {
    type Error = chrono::ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(DateTime::parse_from_rfc3339(&value)?))
    }
}

impl TryFrom<&str> for Timestamp {
    type Error = chrono::ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(DateTime::parse_from_rfc3339(value)?))
    }
}

/// The timestamps for an object.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Timestamps {
    /// The timestamp indicating when the object was created.
    pub created_at: Timestamp,

    /// The timestamp indicating when the object was last updated.
    pub updated_at: Timestamp,
}

#[cfg(test)]
mod test {
    use chrono::DateTime;

    use super::Timestamp;

    #[test]
    fn it_parses_a_timestamp_from_an_iso_string() {
        let iso_string = "2022-06-28T19:07:33.155Z";

        assert_eq!(
            Timestamp::try_from(iso_string),
            DateTime::parse_from_rfc3339(iso_string).map(Timestamp)
        )
    }
}
