mod array;
mod error;
mod number;
mod object;
mod parser;
mod string;
mod tokenizer;
mod value;

use core::ops::Deref;

use alloc::{borrow::Cow, boxed::Box};

use crate::json::parser::JsonParser;

pub use self::{
    array::JsonArray, error::JsonError, number::JsonNumber, object::JsonObject,
    parser::JsonParsingError, string::JsonString, value::JsonValue,
};

#[derive(PartialEq, Eq)]
pub struct Json<'a> {
    data: Cow<'a, str>,
    compiled: Box<[u8]>,
}

impl<'a> Json<'a> {
    /// Build a [`JsonDocument`] from the entire contents.
    pub fn new(data: Cow<'a, str>) -> Result<Self, JsonParsingError> {
        JsonParser::parse(data)
    }
}

impl<'a> Deref for Json<'a> {
    type Target = JsonValue<'a>;

    fn deref(&self) -> &Self::Target {
        unsafe {
            self.compiled
                .as_ptr()
                .cast::<JsonValue>()
                .as_ref_unchecked()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_object() {
        let json_data = "{}";
        let json = Json::new(Cow::Borrowed(json_data)).unwrap();

        let JsonValue::Object(ref object) = *json else {
            panic!("Json did not have Object root.");
        };

        assert_eq!(object.len(), 0);
        assert!(object.is_empty());
    }
}
