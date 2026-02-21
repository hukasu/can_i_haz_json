use core::ops::RangeInclusive;

#[derive(Debug)]
pub struct TokenizerError {
    pub(super) chars: RangeInclusive<usize>,
    pub(super) lines: RangeInclusive<usize>,
    pub(super) column_start: usize,
    pub(super) column_end: usize,
    pub(super) error_type: TokenizerErrorType,
}

#[derive(Debug)]
pub enum TokenizerErrorType {
    UnmatchedCurly,
    UnmatchedBracket,
    InvalidToken,
    InvalidCharacterInString,
    InvalidEscapedCharacterInString,
    UnexpectedEof,
}
