use eyre::{Report, Result};

use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Encoding {
    RGB8,
    BGR8,
    GRAY8,
}

impl Encoding {
    pub fn from_string(encoding: String) -> Result<Encoding> {
        match encoding.as_str() {
            "RGB8" => Ok(Self::RGB8),
            "BGR8" => Ok(Self::BGR8),
            "GRAY8" => Ok(Self::GRAY8),
            _ => Err(Report::msg(format!("Invalid String Encoding {}", encoding))),
        }
    }
}

impl Display for Encoding {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::RGB8 => write!(fmt, "RGB8"),
            Self::BGR8 => write!(fmt, "BGR8"),
            Self::GRAY8 => write!(fmt, "GRAY8"),
        }
    }
}
