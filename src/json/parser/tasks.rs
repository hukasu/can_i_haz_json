use crate::str_cursor::StrCursor;

pub enum JsonParserTask {
    Placeholder,
    Object,
    Array,
    String { start: StrCursor, end: StrCursor },
    Number { start: StrCursor, end: StrCursor },
    True,
    False,
    Null,
}
