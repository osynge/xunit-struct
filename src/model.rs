use crate::errors::XunitError;
use crate::read_xml;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;
use std::convert::From;
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug)]
pub struct TestSuites {
    pub disabled: Option<u32>,
    pub errors: Option<u32>,
    pub failures: Option<u32>,
    pub name: Option<String>,
    pub tests: Option<String>,
    pub time: Option<String>,
    pub testsuite: Vec<TestSuite>,
}

impl TestSuites {
    pub fn try_from_xml(value: &str) -> Result<Self, XunitError> {
        let item: crate::read_xml::TestSuites = from_str(value)?;
        let ts = TestSuites::try_from(item)?;
        Ok(ts)
    }
}

impl TryFrom<crate::read_xml::TestSuites> for TestSuites {
    type Error = XunitError;
    fn try_from(value: crate::read_xml::TestSuites) -> Result<Self, Self::Error> {
        match value {
            crate::read_xml::TestSuites::Testsuites {
                disabled,
                errors,
                failures,
                name,
                tests,
                time,
                testsuite,
            } => {
                // As we want to cascade errors had to expand list
                let mut ts = vec![];
                for val in testsuite.into_iter() {
                    let mut foo = TestSuite::try_from(val)?;
                    ts.push(foo);
                }
                Ok(TestSuites {
                    disabled: disabled,
                    errors: errors,
                    failures: failures,
                    name: name,
                    tests: tests,
                    time: time,
                    testsuite: ts,
                })
            }

            crate::read_xml::TestSuites::Testsuite {
                name,
                tests,
                disabled,
                errors,
                failures,
                hostname,
                id,
                package,
                skipped,
                time,
                timestamp,
                properties,
                testcase,
                system_out,
                system_err,
            } => Err(XunitError::Unknown),
        }
    }
}

#[derive(Debug)]
pub struct TestSuite {
    pub name: String,
    pub tests: u16,
    pub disabled: Option<u32>,
    pub errors: Option<u32>,
    pub failures: Option<u32>,
    pub hostname: Option<String>,
    pub id: Option<String>,
    pub package: Option<String>,
    pub skipped: Option<String>,
    pub time: Option<f32>,
    pub timestamp: Option<DateTime<Utc>>,
    pub properties: Vec<Property>,
    pub testcase: Vec<TestCase>,
    pub system_out: Option<String>,
    pub system_err: Option<String>,
}

impl TryFrom<crate::read_xml::TestSuite> for TestSuite {
    type Error = XunitError;
    fn try_from(value: crate::read_xml::TestSuite) -> Result<Self, Self::Error> {
        let timestamp = match value.timestamp {
            Some(p) => Some(crate::date_time::parse(p.as_str())?),
            None => None,
        };
        let props: Vec<Property> = match value.properties {
            Some(p) => p.value.into_iter().map(|n| Property::from(n)).collect(),
            None => Vec::new(),
        };
        let testcase: Vec<TestCase> = match value.testcase {
            Some(p) => p.into_iter().map(|n| TestCase::from(n)).collect(),
            None => Vec::new(),
        };
        let system_out = match value.system_out {
            Some(p) => Some(p.value),
            None => None,
        };

        let system_err = match value.system_err {
            Some(p) => Some(p.value),
            None => None,
        };
        Ok(TestSuite {
            name: value.name,
            tests: value.tests,
            disabled: value.disabled,
            errors: value.errors,
            failures: value.failures,
            hostname: value.hostname,
            id: value.id,
            package: value.package,
            skipped: value.skipped,
            time: value.time,
            timestamp: timestamp,
            properties: props,
            testcase: testcase,
            system_out: system_out,
            system_err: system_err,
        })
    }
}
#[derive(Debug)]
pub struct TestCase {
    pub assertions: Option<String>,
    pub classname: String,
    pub status: Option<String>,
    pub time: f32,
    pub skipped: Option<String>,
    pub error: Option<Error>,
    pub failure: Option<Failure>,
    pub system_out: Option<String>,
    pub system_err: Option<String>,
}

impl From<crate::read_xml::TestCase> for TestCase {
    fn from(value: crate::read_xml::TestCase) -> Self {
        let error = match value.error {
            Some(p) => Some(Error::from(p)),
            None => None,
        };
        let failure = match value.failure {
            Some(p) => Some(Failure::from(p)),
            None => None,
        };
        let system_out = match value.system_out {
            Some(p) => Some(p.value),
            None => None,
        };
        let system_err = match value.system_err {
            Some(p) => Some(p.value),
            None => None,
        };
        let skipped = match value.skipped {
            Some(p) => Some(p.message),
            None => None,
        };
        TestCase {
            assertions: value.assertions,
            classname: value.classname,
            status: value.status,
            time: value.time,
            skipped: skipped,
            error: error,
            failure: failure,
            system_out: system_out,
            system_err: system_err,
        }
    }
}
#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub error_type: String,
    pub description: String,
}

impl From<crate::read_xml::Error> for Error {
    fn from(value: crate::read_xml::Error) -> Self {
        Error {
            message: value.message,
            error_type: value.error_type,
            description: value.description,
        }
    }
}
#[derive(Debug)]
pub struct Failure {
    pub message: String,
    pub failure_type: String,
    pub description: String,
}

impl From<crate::read_xml::Failure> for Failure {
    fn from(value: crate::read_xml::Failure) -> Self {
        Failure {
            message: value.message,
            failure_type: value.failure_type,
            description: value.description,
        }
    }
}
#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub value: String,
}

impl From<crate::read_xml::Property> for Property {
    fn from(value: crate::read_xml::Property) -> Self {
        Property {
            name: value.name,
            value: value.value,
        }
    }
}

pub struct Item {
    pub name: String,
    pub source: String,
}

impl From<crate::read_xml::Item> for Item {
    fn from(value: crate::read_xml::Item) -> Self {
        Item {
            name: value.name,
            source: value.source,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testsuite_try_from_xml() {
        let junit_str = r#"<?xml version="1.0" encoding="utf-8"?>
<testsuites>
  <testsuite errors="0" failures="0" hostname="e15oms"
  name="pytest" skipped="0" tests="2" time="2.367"
  timestamp="2020-08-28T16:45:10.318141">
    <testcase classname="tests.test_client_owen.Testowen"
    name="test_fm_image" time="0.750"></testcase>
    <testcase classname="tests.test_client_owen.Testowen"
    name="test_fm_video" time="0.756"></testcase>
  </testsuite>
</testsuites>
"#;
        let item = TestSuites::try_from_xml(junit_str).unwrap();
        println!("{:#?}", item);
    }
}
