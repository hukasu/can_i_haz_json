use core::{error::Error, fmt::Display, ops::RangeInclusive};

#[derive(Debug)]
pub struct TokenizerError {
    pub(super) bytes: RangeInclusive<usize>,
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

impl Display for TokenizerError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.error_type {
            TokenizerErrorType::UnmatchedCurly => {
                write!(
                    f,
                    "Unmatched curly brace on {}:{} (bytes {:?}).",
                    self.lines.start(),
                    self.column_start,
                    self.bytes
                )
            }
            TokenizerErrorType::UnmatchedBracket => {
                write!(
                    f,
                    "Unmatched bracket on {}:{} (bytes {:?}).",
                    self.lines.start(),
                    self.column_start,
                    self.bytes
                )
            }
            TokenizerErrorType::InvalidToken => {
                write!(
                    f,
                    "Character position {}:{} (bytes {:?}) is not part of a valid token.",
                    self.lines.start(),
                    self.column_start,
                    self.bytes
                )
            }
            TokenizerErrorType::InvalidCharacterInString => {
                write!(
                    f,
                    "Invalid character in string on position {}:{} (bytes {:?}).",
                    self.lines.start(),
                    self.column_start,
                    self.bytes
                )
            }
            TokenizerErrorType::InvalidEscapedCharacterInString => {
                write!(
                    f,
                    "Invalid escaped character in string on position {}:{} (bytes {:?}).",
                    self.lines.start(),
                    self.column_start,
                    self.bytes
                )
            }
            TokenizerErrorType::UnexpectedEof => {
                write!(f, "Reached end of file while parsing a token.",)
            }
        }
    }
}

impl Error for TokenizerError {}
