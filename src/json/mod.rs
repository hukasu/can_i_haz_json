mod array;
mod data;
mod number;
mod object;
mod parser;
mod string;
mod tokenizer;
mod value;

use core::{borrow::Borrow, error::Error, fmt::Display, ops::Deref, slice::SliceIndex};

pub use self::{
    array::JsonArray,
    number::JsonNumber,
    object::JsonObject,
    parser::JsonParsingError,
    string::JsonString,
    value::{JsonValue, JsonValueType},
};

#[derive(Clone, PartialEq, Eq)]
pub struct JsonDocument<T: Borrow<[u8]> + Clone> {
    contents: T,
}

impl<T: Borrow<[u8]> + Clone> JsonDocument<T> {
    /// Build a [`JsonDocument`] from the entire contents.
    pub fn new(contents: T) -> Result<Self, JsonError> {
        Ok(Self { contents })
    }

    pub fn new_from_range(contents: T, _range: impl SliceIndex<[u8]>) -> Result<Self, JsonError> {
        Ok(Self { contents })
    }
}

impl<T: Borrow<[u8]> + Clone> Deref for JsonDocument<T> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.contents.borrow()
    }
}

#[derive(Debug)]
pub enum JsonError {}

impl Display for JsonError {
    fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Ok(())
    }
}

impl Error for JsonError {}
