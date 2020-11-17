use crate::errors::XunitError;
use chrono::{DateTime, Utc};

pub(crate) fn parse_from_rfc3339(input: &str) -> Result<DateTime<Utc>, XunitError> {
    match DateTime::parse_from_rfc3339(input) {
        Ok(pi) => Ok(DateTime::<Utc>::from(pi)),
        Err(pi) => Err(XunitError::InvalidDate(pi)),
    }
}
