use crate::json::JsonObject;

#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub enum JsonValue<'a> {
    Object(JsonObject<'a>),
    String,
    Number,
    Array,
    True,
    False,
    Null,
}
