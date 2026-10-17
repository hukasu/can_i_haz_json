use core::{error::Error, fmt::Display};

use crate::json::tokenizer::Token;

/// A failure on the parsing of a JSON
#[derive(Debug)]
pub struct JsonParsingError<'a> {
    pub json: &'a str,
    pub token: Token,
    pub error_type: JsonParsingErrorType,
}

#[derive(Debug)]
pub enum JsonParsingErrorType {
    Tokenizer,
    UnexpectedRCurly,
}

impl Display for JsonParsingError<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "Failed to parse JSON.")?;
        writeln!(f)?;
        match self.error_type {
            JsonParsingErrorType::Tokenizer => {
                write!(f, "{:?}", self.token)
            }
            JsonParsingErrorType::UnexpectedRCurly => {
                write!(f, "")
            }
        }
    }
}

impl Error for JsonParsingError<'_> {}
