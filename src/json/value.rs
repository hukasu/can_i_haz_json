use crate::json::JsonObject;

#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub enum JsonValue<'a> {
    Object(JsonObject<'a>),
    Array,
    String(&'a str),
    Number(&'a str),
    True,
    False,
    Null,
}
