#![no_std]

mod object;
mod parser;
mod str_cursor;
mod tokenizer;
mod value;

extern crate alloc;

use core::{fmt::Debug, ops::Deref};

use alloc::{borrow::Cow, boxed::Box};

use crate::parser::JsonParser;

pub use self::{object::JsonObject, parser::JsonParsingError, value::JsonValue};

#[cfg(target_pointer_width = "32")]
const POINTER_WIDTH: usize = 4;
#[cfg(target_pointer_width = "64")]
const POINTER_WIDTH: usize = 8;

const FIRST_OFFSET: usize = POINTER_WIDTH;
const SECOND_OFFSET: usize = FIRST_OFFSET * 2;
const THIRD_OFFSET: usize = FIRST_OFFSET * 3;

#[derive(PartialEq, Eq)]
pub struct Json<'a> {
    data: Cow<'a, str>,
    compiled: Box<[u8]>,
}

impl<'a> Json<'a> {
    /// Build a [`JsonDocument`] from the entire contents.
    pub fn new(data: Cow<'a, str>) -> Result<Self, JsonParsingError<'a>> {
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

impl Debug for Json<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", **self)
    }
}

#[cfg(test)]
mod tests {
    use crate::POINTER_WIDTH;

    use super::*;

    #[test]
    fn empty_object() {
        let json_data = "{}";
        let json = Json::new(Cow::Borrowed(json_data)).unwrap();
        assert_eq!(json.compiled.len(), 0x18);

        let JsonValue::Object(ref object) = *json else {
            panic!("Json did not have Object root.");
        };

        assert_eq!(object.len(), 0);
        assert!(object.is_empty());
    }

    #[test]
    fn empty_array() {
        let json_data = "[]";
        let json = Json::new(Cow::Borrowed(json_data)).unwrap();
        assert_eq!(json.compiled.len(), 0x18);

        let JsonValue::Array { values } = *json else {
            panic!("Json did not have Array root.");
        };
        assert!(values.is_empty())
    }

    #[test]
    fn populated_array() {
        let json_data = "[[\"inner\"], 0, \"hello\", true, false, null]";
        let json = Json::new(Cow::Borrowed(json_data)).unwrap();
        assert_eq!(json.compiled.len(), 0x18 * 8 + 7 * POINTER_WIDTH);

        let JsonValue::Array { values } = *json else {
            panic!("Json did not have Array root.");
        };
        assert_eq!(values.len(), 6);

        let [array, number, string, r#true, r#false, null] = values else {
            panic!("Could not destructure Array values.");
        };

        let JsonValue::Array { values } = *array else {
            panic!("Value did not have Array type.");
        };
        assert_eq!(values.len(), 1);

        let [inner_string] = values else {
            panic!("Could not destructure inner Array values.");
        };

        let JsonValue::String(inner_string) = inner_string else {
            panic!("Inner value did not have String type.");
        };
        assert_eq!(*inner_string, "inner");

        let JsonValue::Number(number) = number else {
            panic!("Value did not have Number type.");
        };
        assert_eq!(*number, "0");

        let JsonValue::String(string) = string else {
            panic!("Value did not have String type.");
        };
        assert_eq!(*string, "hello");

        let JsonValue::True = r#true else {
            panic!("Value did not have True type.");
        };

        let JsonValue::False = r#false else {
            panic!("Value did not have False type.");
        };

        let JsonValue::Null = null else {
            panic!("Value did not have Null type.");
        };
    }

    #[test]
    fn empty_string() {
        let json_data = "\"\"";
        let json = Json::new(Cow::Borrowed(json_data)).unwrap();
        assert_eq!(json.compiled.len(), 0x18);

        let JsonValue::String(string) = *json else {
            panic!("Json did not have String root.");
        };

        assert_eq!(string, "");
    }

    #[test]
    fn string() {
        let json_data = "\"string\"";
        let json = Json::new(Cow::Borrowed(json_data)).unwrap();
        assert_eq!(json.compiled.len(), 0x18);

        let JsonValue::String(string) = *json else {
            panic!("Json did not have String root.");
        };

        assert_eq!(string, "string");
    }

    #[test]
    fn zero() {
        let json_data = "0";
        let json = Json::new(Cow::Borrowed(json_data)).unwrap();
        assert_eq!(json.compiled.len(), 0x18);

        let JsonValue::Number(number) = *json else {
            panic!("Json did not have Number root.");
        };

        assert_eq!(number.parse::<f32>(), Ok(0.0));
    }

    #[test]
    fn true_root() {
        let json_data = "true";
        let json = Json::new(Cow::Borrowed(json_data)).unwrap();
        assert_eq!(json.compiled.len(), 0x18);

        let JsonValue::True = *json else {
            panic!("Json did not have True root.");
        };
    }

    #[test]
    fn false_root() {
        let json_data = "false";
        let json = Json::new(Cow::Borrowed(json_data)).unwrap();
        assert_eq!(json.compiled.len(), 0x18);

        let JsonValue::False = *json else {
            panic!("Json did not have False root.");
        };
    }

    #[test]
    fn null_root() {
        let json_data = "null";
        let json = Json::new(Cow::Borrowed(json_data)).unwrap();
        assert_eq!(json.compiled.len(), 0x18);

        let JsonValue::Null = *json else {
            panic!("Json did not have Null root.");
        };
    }
}
