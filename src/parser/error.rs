use core::{error::Error, fmt::Display};

use alloc::borrow::Cow;

use crate::{
    str_cursor::StrCursor,
    tokenizer::{TokenType, TokenizerErrorType},
};

/// A failure on the parsing of a JSON
#[derive(Debug)]
pub struct JsonParsingError<'a> {
    pub json: Cow<'a, str>,
    pub start: StrCursor,
    pub end: StrCursor,
    pub error_type: JsonParsingErrorType,
}

#[derive(Debug)]
pub enum JsonParsingErrorType {
    Tokenizer(TokenizerErrorType),
    UnexpectedRCurly,
    UnmatchedRCurly,
    UnexpectedRBracket,
    UnmatchedRBracket,
    UnexpectedComma,
    NoValueAfterArraySeparator,
    NoCommaBetweeArrayValues,
    InvalidObjectKey(TokenType),
    CommaBeforeObjectKey,
    CommaBeforeObjectKeyValue,
    ObjectExpectedComma,
    UnclosedObject,
    UnclosedArray,
    MultipleRoots,
    EmptyBody,
}

impl Display for JsonParsingError<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "Failed to parse JSON.")?;
        writeln!(f)?;
        match self.error_type {
            JsonParsingErrorType::Tokenizer(_) => {
                write!(f, "{:?}:{:?}", self.start, self.end)
            }
            JsonParsingErrorType::UnexpectedRCurly => {
                write!(f, "")
            }
            JsonParsingErrorType::UnmatchedRCurly => {
                write!(f, "")
            }
            JsonParsingErrorType::UnexpectedRBracket => {
                write!(f, "")
            }
            JsonParsingErrorType::UnmatchedRBracket => {
                write!(f, "")
            }
            JsonParsingErrorType::UnexpectedComma => {
                write!(f, "")
            }
            JsonParsingErrorType::NoValueAfterArraySeparator => {
                write!(f, "")
            }
            JsonParsingErrorType::NoCommaBetweeArrayValues => {
                write!(f, "")
            }
            JsonParsingErrorType::InvalidObjectKey(_) => {
                write!(f, "")
            }
            JsonParsingErrorType::CommaBeforeObjectKey => {
                write!(f, "")
            }
            JsonParsingErrorType::CommaBeforeObjectKeyValue => {
                write!(f, "")
            }
            JsonParsingErrorType::ObjectExpectedComma => {
                write!(f, "")
            }
            JsonParsingErrorType::UnclosedObject => {
                write!(f, "")
            }
            JsonParsingErrorType::UnclosedArray => {
                write!(f, "")
            }
            JsonParsingErrorType::MultipleRoots => {
                write!(f, "")
            }
            JsonParsingErrorType::EmptyBody => {
                write!(f, "")
            }
        }
    }
}

impl Error for JsonParsingError<'_> {}
