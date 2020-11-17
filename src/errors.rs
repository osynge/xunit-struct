use thiserror::Error;

#[derive(Error, Debug)]
pub enum XunitError {
    #[error("xml parsing error")]
    Xml(#[from] serde_xml_rs::Error),
    #[error("Could not parse date")]
    InvalidDate(#[from] chrono::ParseError),
    #[error("unknown xUnit error")]
    Unknown,
}
