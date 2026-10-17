use crate::json::JsonValue;

#[derive(Debug, PartialEq, Eq)]
pub struct JsonObject<'a> {
    pairs: &'a [JsonKeyValue<'a>],
}

#[derive(Debug, PartialEq, Eq)]
pub struct JsonKeyValue<'a> {
    key: &'a str,
    value: &'a JsonValue<'a>,
}
