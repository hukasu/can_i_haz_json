use crate::json::JsonObject;

#[derive(Debug, PartialEq, Eq)]
pub enum JsonValue<'a> {
    Object(JsonObject<'a>),
    String,
    Number,
    Array,
    True,
    False,
    Null,
}
