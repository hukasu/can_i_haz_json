use core::{error::Error, fmt::Display};

use crate::json::{JsonParsingError, tokenizer::TokenizerError};

#[derive(Debug)]
pub enum JsonError<'a> {
    Tokenizer(TokenizerError),
    Parsing(JsonParsingError<'a>),
}

impl Display for JsonError<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Tokenizer(tokenizer) => {
                writeln!(f, "Failed to tokenize Json.")?;
                write!(f, "{tokenizer}")
            }
            Self::Parsing(parser) => {
                writeln!(f, "Failed to parse Json.")?;
                write!(f, "{parser}")
            }
        }
    }
}

impl Error for JsonError {}

impl From<TokenizerError> for JsonError {
    fn from(value: TokenizerError) -> Self {
        Self::Tokenizer(value)
    }
}

impl From<JsonParsingError> for JsonError {
    fn from(value: JsonParsingError) -> Self {
        match value {
            JsonParsingError::Tokenizer(value) => Self::Tokenizer(value),
            other => Self::Parsing(other),
        }
    }
}
