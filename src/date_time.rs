use crate::errors::XunitError;
use chrono::{DateTime, NaiveDateTime};

static DATE_TIME_FORMAT_PYTEST: &str = "%FT%T%.6f";

pub(crate) fn parse_from_native(input: &str) -> Result<i64, XunitError> {
    match NaiveDateTime::parse_from_str(input, DATE_TIME_FORMAT_PYTEST) {
        Ok(pi) => Ok(pi.timestamp()),
        Err(pi) => Err(XunitError::InvalidDate(pi)),
    }
}

pub(crate) fn parse(input: &str) -> Result<i64, XunitError> {
    if let Ok(pi) = DateTime::parse_from_rfc3339(input) {
        return Ok(pi.timestamp());
    }
    parse_from_native(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_pytest_timestamp() {
        let ts = "2020-09-07T16:03:44.192592";
        parse(&ts).unwrap();
    }

    #[test]
    fn test_parse_native() {
        let ts = "2020-09-07T16:03:44.192592";
        parse_from_native(&ts).unwrap();
    }
}
