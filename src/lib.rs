#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate log;
extern crate serde_xml_rs;
extern crate simple_logger;

mod date_time;
pub mod errors;
pub mod model;
mod read_xml;
