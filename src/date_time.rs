use crate::errors::XunitError;
use chrono::format::strftime;
use chrono::{DateTime, NaiveDateTime, Utc};

static DATE_TIME_FORMAT_PYTEST: &'static str = "%FT%T%.6f";

pub(crate) fn parse_from_native(input: &str) -> Result<DateTime<Utc>, XunitError> {
    match NaiveDateTime::parse_from_str(input, DATE_TIME_FORMAT_PYTEST) {
        Ok(pi) => Ok(DateTime::<Utc>::from_utc(pi, Utc)),
        Err(pi) => Err(XunitError::InvalidDate(pi)),
    }
}

pub(crate) fn parse(input: &str) -> Result<DateTime<Utc>, XunitError> {
    match DateTime::parse_from_rfc3339(input) {
        Ok(pi) => return Ok(DateTime::<Utc>::from(pi)),
        Err(_) => (),
    };
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
