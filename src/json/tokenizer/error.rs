use core::{error::Error, fmt::Display};

use crate::str_cursor::StrCursor;

#[derive(Debug)]
pub struct TokenizerError {
    pub(super) start: StrCursor,
    pub(super) end: StrCursor,
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
                    self.start.line, self.start.column, self.start.byte
                )
            }
            TokenizerErrorType::UnmatchedBracket => {
                write!(
                    f,
                    "Unmatched bracket on {}:{} (bytes {:?}).",
                    self.start.line, self.start.column, self.start.byte,
                )
            }
            TokenizerErrorType::InvalidToken => {
                write!(
                    f,
                    "Character position {}:{} (bytes {:?}) is not part of a valid token.",
                    self.start.line, self.start.column, self.start.byte,
                )
            }
            TokenizerErrorType::InvalidCharacterInString => {
                write!(
                    f,
                    "Invalid character in string on position {}:{} (bytes {:?}).",
                    self.start.line, self.start.column, self.start.byte,
                )
            }
            TokenizerErrorType::InvalidEscapedCharacterInString => {
                write!(
                    f,
                    "Invalid escaped character in string on position {}:{} (bytes {:?}).",
                    self.start.line, self.start.column, self.start.byte,
                )
            }
            TokenizerErrorType::UnexpectedEof => {
                write!(f, "Reached end of file while parsing a token.",)
            }
        }
    }
}

impl Error for TokenizerError {}
