mod array;
mod data;
mod number;
mod object;
mod parser;
mod string;
mod tokenizer;
mod value;

use core::{error::Error, fmt::Display, ops::Deref};

use alloc::vec::Vec;

use crate::json::tokenizer::{JsonTokenizer, TokenizerError};

pub use self::{
    array::JsonArray,
    number::JsonNumber,
    object::JsonObject,
    parser::JsonParsingError,
    string::JsonString,
    value::{JsonValue, JsonValueType},
};

#[derive(Clone, PartialEq, Eq)]
pub struct JsonDocument<T: Sized + Deref<Target = str> + Clone> {
    contents: T,
}

impl JsonDocument<&str> {
    pub const BLANK: Self = JsonDocument { contents: "" };
}

impl<T: Deref<Target = str> + Clone + Unpin> JsonDocument<T> {
    /// Build a [`JsonDocument`] from the entire contents.
    pub fn new(contents: T) -> Result<Self, JsonError> {
        let data = contents.deref();

        let tokens =
            JsonTokenizer::new(data.chars()).collect::<Result<Vec<_>, TokenizerError>>()?;

        Ok(Self { contents })
    }
}

#[derive(Debug)]
pub enum JsonError {
    Tokenizer(TokenizerError),
}

impl Display for JsonError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Tokenizer(tokenizer) => {
                writeln!(f, "Failed to tokenize Json.")?;
                write!(f, "{tokenizer}")
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
