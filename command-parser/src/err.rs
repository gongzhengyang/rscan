use std::error::Error;
use std::fmt::{Display, Formatter};

use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub enum APPError {
    PortFormatError,
}

impl Display for APPError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::json!(&self))
    }
}

impl Error for APPError {}
