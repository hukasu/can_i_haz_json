pub struct JsonValue {
    r#type: JsonValueType,
    index: usize,
}

#[derive(Debug)]
pub enum JsonValueType {
    String,
    Number,
    Object,
    Array,
    Bool,
    Null,
}
