mod array;
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
    array::JsonArray, number::JsonNumber, object::JsonObject, parser::JsonParsingError,
    string::JsonString, value::JsonValue,
};

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

#[cfg(test)]
mod tests {
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

        let JsonValue::Array = *json else {
            panic!("Json did not have Array root.");
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
