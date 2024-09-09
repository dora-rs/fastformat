use eyre::{Report, Result};

use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Encoding {
    Logical,
    Physical,
}

impl Encoding {
    pub fn from_string(encoding: String) -> Result<Encoding> {
        match encoding.as_str() {
            "Logical" => Ok(Self::Logical),
            "Physical" => Ok(Self::Physical),
            _ => Err(Report::msg(format!("Invalid String Encoding {}", encoding))),
        }
    }
}

impl Display for Encoding {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Logical => write!(fmt, "Logical"),
            Self::Physical => write!(fmt, "Physical"),
        }
    }
}
