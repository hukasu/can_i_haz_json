use alloc::vec::Vec;

use crate::str_cursor::StrCursor;

pub enum JsonParserTask {
    Placeholder,
    Object { pairs: Vec<(usize, usize)> },
    Array { values: Vec<usize> },
    String { start: StrCursor, end: StrCursor },
    Number { start: StrCursor, end: StrCursor },
    True,
    False,
    Null,
}
