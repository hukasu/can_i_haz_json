use core::{error::Error, fmt::Display};

use crate::json::tokenizer::TokenizerError;

/// A failure on the parsing of a JSON
#[derive(Debug)]
pub enum JsonParsingError {
    Tokenizer(TokenizerError),
}

impl From<TokenizerError> for JsonParsingError {
    fn from(value: TokenizerError) -> Self {
        Self::Tokenizer(value)
    }
}

impl Display for JsonParsingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "Failed to parse JSON.")?;
        writeln!(f)?;
        match self {
            JsonParsingError::Tokenizer(tokenizer_error) => {
                write!(f, "{tokenizer_error}")
            }
        }
    }
}

impl Error for JsonParsingError {}
