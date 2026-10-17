mod array;
mod data;
mod error;
mod number;
mod object;
mod parser;
mod string;
mod tokenizer;
mod value;

use alloc::borrow::Cow;

use crate::json::{parser::JsonParser, tokenizer::JsonTokenizer};

pub use self::{
    array::JsonArray, error::JsonError, number::JsonNumber, object::JsonObject,
    parser::JsonParsingError, string::JsonString, value::JsonValue,
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
        unimplemented!()
    }
}
