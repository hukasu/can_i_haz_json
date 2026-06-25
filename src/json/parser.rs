use core::{error::Error, fmt::Display};

use alloc::vec::Vec;

use crate::json::{
    JsonValue,
    tokenizer::{JsonTokenizer, TokenType, TokenizerError},
};

pub struct JsonParser;

impl JsonParser {
    pub fn parse<T: Iterator<Item = char>>(
        tokens: JsonTokenizer<T>,
    ) -> Result<JsonValue, JsonParsingError> {
        // let
        let mut stack = alloc::vec![];

        for token in tokens {
            let token = token?;
            match token.token_type {
                TokenType::String => match stack.last_mut() {
                    Some(JsonValue::Array) => {
                        unimplemented!()
                    }
                    None => {}
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
        }

        unimplemented!()
    }
}

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
