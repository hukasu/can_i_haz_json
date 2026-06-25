/// A cursor position into a [`str`].
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct StrCursor {
    pub byte: usize,
    pub line: usize,
    pub column: usize,
}

impl StrCursor {
    /// Create a [`Range<usize>`](core::range::Range) from this objects
    /// [`byte`](StrCursor::byte) to the `end`s `byte`.
    pub fn to_range(&self, end: &Self) -> core::range::Range<usize> {
        (self.byte..end.byte).into()
    }
}

impl Default for StrCursor {
    fn default() -> Self {
        Self {
            byte: 0,
            line: 1,
            column: 1,
        }
    }
}
