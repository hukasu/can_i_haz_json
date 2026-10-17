use crate::str_cursor::StrCursor;

pub enum JsonParserTask {
    Object,
    String { start: StrCursor, end: StrCursor },
}
