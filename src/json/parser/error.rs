use core::{error::Error, fmt::Display};

use alloc::borrow::Cow;

use crate::{json::tokenizer::TokenizerErrorType, str_cursor::StrCursor};

/// A failure on the parsing of a JSON
#[derive(Debug)]
pub struct JsonParsingError<'a> {
    pub json: Cow<'a, str>,
    pub start: StrCursor,
    pub end: StrCursor,
    pub error_type: JsonParsingErrorType,
}

#[derive(Debug)]
pub enum JsonParsingErrorType {
    Tokenizer(TokenizerErrorType),
    UnexpectedRCurly,
}

impl Display for JsonParsingError<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "Failed to parse JSON.")?;
        writeln!(f)?;
        match self.error_type {
            JsonParsingErrorType::Tokenizer(_) => {
                write!(f, "{:?}:{:?}", self.start, self.end)
            }
            JsonParsingErrorType::UnexpectedRCurly => {
                write!(f, "")
            }
        }
    }
}

impl Error for JsonParsingError<'_> {}
