use alloc::{boxed::Box, rc::Rc, sync::Arc};
use core::{error::Error, fmt::Display};

use crate::json::{
    JsonDocument,
    data::JsonData,
    tokenizer::{JsonTokenizer, TokenizerError},
};

impl JsonDocument<Box<JsonData>> {
    pub fn parse(contents: &str) -> Result<Self, JsonParsingError> {
        let tokens = JsonTokenizer::new(contents.chars()).tokenize()?;
        Ok(Self {
            data: Box::new(JsonData::default()),
        })
    }
}

impl JsonDocument<Rc<JsonData>> {
    pub fn parse(contents: &str) -> Result<Self, JsonParsingError> {
        let tokens = JsonTokenizer::new(contents.chars()).tokenize()?;
        Ok(Self {
            data: Rc::new(JsonData::default()),
        })
    }
}

impl JsonDocument<Arc<JsonData>> {
    pub fn parse(contents: &str) -> Result<Self, JsonParsingError> {
        let tokens = JsonTokenizer::new(contents.chars()).tokenize()?;
        Ok(Self {
            data: Arc::new(JsonData::default()),
        })
    }
}

// struct JsonParser<T: Iterator<Item = char>> {}

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
