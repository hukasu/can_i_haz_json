use core::{error::Error, fmt::Display};

use crate::str_cursor::StrCursor;

#[derive(Debug)]
pub struct TokenizerError {
    pub start: StrCursor,
    pub end: StrCursor,
    pub error_type: TokenizerErrorType,
}

#[derive(Debug)]
pub enum TokenizerErrorType {
    InvalidToken,
    InvalidCharacterInString,
    InvalidEscapedCharacterInString,
    InvalidDigitInNumber,
    DigitAfterZero,
    NoDigitAfterFraction,
    UnexpectedEof,
}

impl Display for TokenizerError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.error_type {
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
            TokenizerErrorType::InvalidDigitInNumber => {
                write!(
                    f,
                    "Invalid digit in number at position {}:{} (bytes {:?}).",
                    self.start.line, self.start.column, self.start.byte,
                )
            }
            TokenizerErrorType::DigitAfterZero => {
                write!(
                    f,
                    "Invalid digit after a zero at position {}:{} (bytes {:?}).",
                    self.start.line, self.start.column, self.start.byte,
                )
            }
            TokenizerErrorType::NoDigitAfterFraction => {
                write!(
                    f,
                    "No digits after fraction at position {}:{} (bytes {:?}).",
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
