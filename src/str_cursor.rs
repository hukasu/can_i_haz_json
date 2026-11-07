/// A cursor position into a [`str`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StrCursor {
    pub byte: usize,
    pub line: usize,
    pub column: usize,
}

impl StrCursor {
    /// Create a [`Range<usize>`](core::range::Range) from this objects
    /// [`byte`](StrCursor::byte) to the `end`s `byte`.
    pub fn to_range(self, end: &Self) -> core::range::Range<usize> {
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

impl core::ops::Add<char> for StrCursor {
    type Output = StrCursor;

    fn add(self, rhs: char) -> Self::Output {
        let new_line = rhs == '\n';
        Self {
            byte: self.byte + rhs.len_utf8(),
            line: self.line + new_line as usize,
            column: if new_line { 1 } else { self.column + 1 },
        }
    }
}

impl core::ops::AddAssign<char> for StrCursor {
    fn add_assign(&mut self, rhs: char) {
        *self = *self + rhs;
    }
}
