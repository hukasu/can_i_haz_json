#[derive(PartialEq, Eq)]
pub enum JsonValue {
    String,
    Number,
    Object,
    Array,
    True,
    False,
    Null,
}
