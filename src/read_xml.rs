#[derive(Debug, Deserialize, PartialEq)]
pub(crate) enum TestSuites {
    #[serde(rename = "testsuites")]
    Testsuites {
        disabled: Option<u32>,
        errors: Option<u32>,
        failures: Option<u32>,
        name: Option<String>,
        tests: Option<u32>,
        time: Option<String>,
        testsuite: Vec<TestSuite>,
    },
    #[serde(rename = "testsuite")]
    Testsuite {
        name: String,
        tests: u32,
        disabled: Option<u32>,
        errors: Option<u32>,
        failures: Option<u32>,
        hostname: Option<String>,
        id: Option<String>,
        package: Option<String>,
        skipped: Option<String>,
        time: Option<f32>,
        timestamp: Option<String>,
        properties: Option<Properties>,
        testcase: Option<Vec<TestCase>>,
        #[serde(rename = "system-out")]
        system_out: Option<SystemOut>,
        #[serde(rename = "system-err>")]
        system_err: Option<SystemErr>,
    },
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "testsuite")]
pub(crate) struct TestSuite {
    pub(crate) name: String,
    pub(crate) tests: Option<u32>,
    pub(crate) disabled: Option<u32>,
    pub(crate) errors: Option<u32>,
    pub(crate) failures: Option<u32>,
    pub(crate) hostname: Option<String>,
    pub(crate) id: Option<String>,
    pub(crate) package: Option<String>,
    pub(crate) skipped: Option<String>,
    pub(crate) time: Option<f32>,
    pub(crate) timestamp: Option<String>,
    pub(crate) properties: Option<Properties>,
    pub(crate) testcase: Option<Vec<TestCase>>,
    #[serde(rename = "system-out")]
    pub(crate) system_out: Option<SystemOut>,
    #[serde(rename = "system-err>")]
    pub(crate) system_err: Option<SystemErr>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "testcase")]
pub(crate) struct TestCase {
    pub(crate) name: String,
    pub(crate) assertions: Option<String>,
    pub(crate) classname: String,
    pub(crate) status: Option<String>,
    pub(crate) time: f32,
    pub(crate) skipped: Option<Skipped>,
    pub(crate) error: Option<Error>,
    pub(crate) failure: Option<Failure>,
    #[serde(rename = "system-out", default)]
    pub(crate) system_out: Option<SystemOut>,
    #[serde(rename = "system-err", default)]
    pub(crate) system_err: Option<SystemErr>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "skipped")]
pub(crate) struct Skipped {
    pub(crate) message: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "error")]
pub(crate) struct Error {
    pub(crate) message: String,
    #[serde(rename = "type", default)]
    pub(crate) error_type: String,
    #[serde(rename = "$value")]
    pub(crate) description: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "failure")]
pub(crate) struct Failure {
    pub(crate) message: String,
    #[serde(rename = "type", default)]
    pub(crate) failure_type: String,
    #[serde(rename = "$value")]
    pub(crate) description: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "system-out")]
pub(crate) struct SystemOut {
    #[serde(rename = "$value")]
    pub(crate) value: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "system-err")]
pub(crate) struct SystemErr {
    #[serde(rename = "$value")]
    pub(crate) value: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "property")]
pub(crate) struct Property {
    pub(crate) name: String,
    pub(crate) value: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct Properties {
    #[serde(rename = "$value")]
    pub(crate) value: Vec<Property>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct Item {
    pub(crate) name: String,
    pub(crate) source: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_xml_rs::from_str;

    #[test]
    fn failure() {
        let junit_str = r#"<failure message="" type=""
	       >failure description</failure>"#;
        let item: Failure = from_str(junit_str).unwrap();
        println!("{:#?}", item);
    }

    #[test]
    fn testcase() {
        let junit_str = r#"<testcase name="name"
	      assertions="true = 1"
	      classname="testclass"
	      status="status"
	      time="0.1"
          >
           <error message="The error message. e.g., if a java exception is thrown, the return value of getMessage()"
	     type="The type of error that occured. e.g., if a java execption is thrown the full class name of the exception."
	     >error description</error>
          <failure message="The message specified in the assert."
	       type="The type of the assert."
	       >failure description</failure>
          <skipped message="message/description string why the test case was skipped. optional"/>
          <system-out>STDOUT text</system-out>
           <system-err>STDERR text</system-err>
          </testcase>
          "#;
        let item: TestCase = from_str(junit_str).unwrap();
        println!("{:#?}", item);
    }

    #[test]
    fn properties() {
        let junit_str = r#"<properties>
            <property name="name" value="value"/>
        </properties>"#;
        let item: Properties = from_str(junit_str).unwrap();
        println!("{:#?}", item);
    }

    #[test]
    fn testsuites() {
        let junit_str = r#"<?xml version="1.0" encoding="UTF-8"?>
        <testsuites name="with testsuites" >
            <testsuite name="name" tests="3">
                <testcase assertions=""classname="" status="" time="1" name="dfsf">
                </testcase>
            </testsuite>
        </testsuites>"#;
        let item: TestSuites = from_str(junit_str).unwrap();
        println!("{:#?}", item);
    }

    #[test]
    fn testsuite() {
        let junit_str = r#"<?xml version="1.0" encoding="utf-8"?>
<testsuites>
  <testsuite errors="0" failures="0" hostname="e15oms"
  name="pytest" skipped="0" tests="2" time="2.367"
  timestamp="2020-08-28T16:45:10.318141">
    <testcase classname="tests.test_fm_client_owen.Testowen"
    name="test_fm_image" time="0.750"></testcase>
    <testcase classname="tests.test_fm_client_owen.Testowen"
    name="test_fm_video" time="0.756"></testcase>
  </testsuite>
</testsuites>
"#;
        let item: TestSuites = from_str(junit_str).unwrap();
        println!("{:#?}", item);
    }

    #[test]
    fn full() {
        let junit_str = std::include_str!("../tests/data/reference.xml");
        let jam = junit_str
            .split_inclusive("-->")
            .map(|substring| substring.split("<!--").next().unwrap_or(""))
            .collect::<String>()
            .replace("time=\"\"", "time=\"0.1\"")
            .replace("disabled=\"\"", "disabled=\"1\"")
            .replace("failures=\"\"", "failures=\"1\"")
            .replace("errors=\"\"", "errors=\"1\"")
            .replace("tests=\"\"", "tests=\"1\"");
        for line in jam.lines() {
            println!("{}", line);
        }

        let item: TestSuites = match from_str(&jam) {
            Ok(p) => p,
            Err(p) => {
                println!("deserialise error:{:?}", p);
                panic!("fdfd")
            }
        };
        println!("{:?}", item);
    }
}
