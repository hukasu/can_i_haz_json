mod array;
mod data;
mod number;
mod object;
mod parser;
mod string;
mod tokenizer;
mod value;

use core::{error::Error, fmt::Display};

use alloc::borrow::Cow;

use crate::json::{
    parser::JsonParser,
    tokenizer::{JsonTokenizer, TokenizerError},
};

pub use self::{
    array::JsonArray, number::JsonNumber, object::JsonObject, parser::JsonParsingError,
    string::JsonString, value::JsonValue,
};

#[derive(PartialEq, Eq)]
pub struct Json<'a> {
    data: Cow<'a, str>,
    root: JsonValue,
}

impl<'a> Json<'a> {
    /// Build a [`JsonDocument`] from the entire contents.
    pub fn new(data: Cow<'a, str>) -> Result<Self, JsonParsingError> {
        let tokens = JsonTokenizer::new(data.chars());
        let root = JsonParser::parse(tokens)?;
        Ok(Self { data, root })
    }
}

#[derive(Debug)]
pub enum JsonError {
    Tokenizer(TokenizerError),
    Parsing(JsonParsingError),
}

impl Display for JsonError {
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
