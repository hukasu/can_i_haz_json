use alloc::vec::Vec;

use crate::str_cursor::StrCursor;

pub enum JsonParserStack {
    Object {
        start: StrCursor,
        placeholder: usize,
        pairs: Vec<(usize, usize)>,
    },
    Array {
        start: StrCursor,
        placeholder: usize,
        values: Vec<usize>,
    },
}
