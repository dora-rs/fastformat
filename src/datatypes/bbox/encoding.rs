use eyre::{Report, Result};

use std::fmt::Display;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
pub enum Encoding {
    XYXY,
    XYWH,
}

impl Encoding {
    pub fn from_string(encoding: String) -> Result<Encoding> {
        match encoding.as_str() {
            "XYXY" => Ok(Self::XYXY),
            "XYWH" => Ok(Self::XYWH),
            _ => Err(Report::msg(format!("Invalid String Encoding {}", encoding))),
        }
    }
}

impl Display for Encoding {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::XYXY => write!(fmt, "XYXY"),
            Self::XYWH => write!(fmt, "XYWH"),
        }
    }
}
