#![expect(dead_code)]

mod error;
#[cfg(test)]
mod tests;

use alloc::vec::Vec;
use core::ops::RangeInclusive;

use crate::json::tokenizer::error::TokenizerErrorType;

pub use self::error::TokenizerError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    chars: RangeInclusive<usize>,
    lines: RangeInclusive<usize>,
    column_start: usize,
    column_end: usize,
    token_type: TokenType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    LCurly,
    RCurly,
    LBracket,
    RBracket,
    Colon,
    Comma,
    String,
    Number,
    True,
    False,
    Null,
    Eof,
}

#[expect(type_alias_bounds)]
type State<T: Iterator<Item = char>> =
    fn(&mut JsonTokenizer<T>) -> Result<Option<Token>, TokenizerError>;

pub struct JsonTokenizer<T: Iterator<Item = char>> {
    iterator: T,
    r#char: usize,
    char_start: usize,
    column: usize,
    column_start: usize,
    line: usize,
    line_start: usize,
    state: State<T>,
    fused: bool,
}

impl<T: Iterator<Item = char>> JsonTokenizer<T> {
    pub fn new(iterator: T) -> Self {
        Self {
            iterator,
            char: 0,
            char_start: 0,
            column: 1,
            column_start: 1,
            line: 1,
            line_start: 1,
            state: Self::start,
            fused: false,
        }
    }

    pub fn tokenize(self) -> Result<Vec<Token>, TokenizerError> {
        self.collect()
    }

    fn start(&mut self) -> Result<Option<Token>, TokenizerError> {
        if self.fused {
            return self.eof();
        }
        let Some(current_char) = self.iterator.next() else {
            return self.eof();
        };

        let res = match current_char {
            '\u{0020}' | '\u{000D}' | '\u{0009}' => Ok(None),
            '\u{000A}' => {
                // Zero because it will be added by 1 at the end of the method
                self.column = 0;
                self.line += 1;
                Ok(None)
            }
            '{' => Ok(Some(Token {
                chars: self.char..=self.char,
                lines: self.line..=self.line,
                column_start: self.column,
                column_end: self.column + 1,
                token_type: TokenType::LCurly,
            })),
            '}' => Ok(Some(Token {
                chars: self.char..=self.char,
                lines: self.line..=self.line,
                column_start: self.column,
                column_end: self.column + 1,
                token_type: TokenType::RCurly,
            })),
            '[' => Ok(Some(Token {
                chars: self.char..=self.char,
                lines: self.line..=self.line,
                column_start: self.column,
                column_end: self.column + 1,
                token_type: TokenType::LBracket,
            })),
            ']' => Ok(Some(Token {
                chars: self.char..=self.char,
                lines: self.line..=self.line,
                column_start: self.column,
                column_end: self.column + 1,
                token_type: TokenType::RBracket,
            })),
            ',' => Ok(Some(Token {
                chars: self.char..=self.char,
                lines: self.line..=self.line,
                column_start: self.column,
                column_end: self.column + 1,
                token_type: TokenType::Comma,
            })),
            ':' => Ok(Some(Token {
                chars: self.char..=self.char,
                lines: self.line..=self.line,
                column_start: self.column,
                column_end: self.column + 1,
                token_type: TokenType::Colon,
            })),
            '"' => {
                self.char_start = self.char;
                self.column_start = self.column + 1;
                self.line_start = self.line;
                self.state = Self::string;
                Ok(None)
            }
            _ => self.invalid_token(),
        };

        self.char += 1;
        self.column += 1;

        res
    }

    fn string(&mut self) -> Result<Option<Token>, TokenizerError> {
        let Some(current_char) = self.iterator.next() else {
            return self.unexpected_eof();
        };

        let res = match current_char {
            '"' => {
                self.state = Self::start;
                Ok(Some(Token {
                    chars: self.char_start..=self.char,
                    lines: self.line_start..=self.line,
                    column_start: self.column_start,
                    column_end: self.column,
                    token_type: TokenType::String,
                }))
            }
            '\\' => {
                self.state = Self::escaped_character;
                Ok(None)
            }
            '\u{0020}'..='\u{10ffff}' => Ok(None),
            _ => self.invalid_character_in_string(),
        };

        self.char += 1;
        self.column += 1;

        res
    }

    fn escaped_character(&mut self) -> Result<Option<Token>, TokenizerError> {
        let Some(current_char) = self.iterator.next() else {
            return self.unexpected_eof();
        };

        let res = match current_char {
            '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't' => {
                self.state = Self::string;
                Ok(None)
            }
            'u' => {
                self.state = Self::hex::<0>;
                Ok(None)
            }
            _ => self.invalid_escaped_character_in_string(),
        };

        self.char += 1;
        self.column += 1;

        res
    }

    fn hex<const N: usize>(&mut self) -> Result<Option<Token>, TokenizerError> {
        let Some(current_char) = self.iterator.next() else {
            return self.unexpected_eof();
        };

        self.state = if N == 0 {
            Self::hex::<1>
        } else if N == 1 {
            Self::hex::<2>
        } else if N == 2 {
            Self::hex::<3>
        } else {
            Self::string
        };

        let res = if current_char.is_ascii_hexdigit() {
            Ok(None)
        } else {
            self.invalid_escaped_character_in_string()
        };

        self.char += 1;
        self.column += 1;

        res
    }

    #[inline(always)]
    fn eof(&self) -> Result<Option<Token>, TokenizerError> {
        Ok(Some(Token {
            chars: self.char..=self.char,
            lines: self.line..=self.line,
            column_start: self.column,
            column_end: self.column + 1,
            token_type: TokenType::Eof,
        }))
    }

    #[inline(always)]
    fn unexpected_eof(&self) -> Result<Option<Token>, TokenizerError> {
        Err(TokenizerError {
            chars: self.char..=self.char,
            lines: self.line..=self.line,
            column_start: self.column,
            column_end: self.column + 1,
            error_type: TokenizerErrorType::UnexpectedEof,
        })
    }

    #[inline(always)]
    fn unmatched_curly(&self) -> Result<Option<Token>, TokenizerError> {
        Err(TokenizerError {
            chars: self.char..=self.char,
            lines: self.line..=self.line,
            column_start: self.column,
            column_end: self.column + 1,
            error_type: TokenizerErrorType::UnmatchedCurly,
        })
    }

    #[inline(always)]
    fn unmatched_bracket(&self) -> Result<Option<Token>, TokenizerError> {
        Err(TokenizerError {
            chars: self.char..=self.char,
            lines: self.line..=self.line,
            column_start: self.column,
            column_end: self.column + 1,
            error_type: TokenizerErrorType::UnmatchedBracket,
        })
    }
    #[inline(always)]
    fn invalid_token(&self) -> Result<Option<Token>, TokenizerError> {
        Err(TokenizerError {
            chars: self.char..=self.char,
            lines: self.line..=self.line,
            column_start: self.column,
            column_end: self.column + 1,
            error_type: TokenizerErrorType::InvalidToken,
        })
    }

    #[inline(always)]
    fn invalid_character_in_string(&self) -> Result<Option<Token>, TokenizerError> {
        Err(TokenizerError {
            chars: self.char..=self.char,
            lines: self.line..=self.line,
            column_start: self.column,
            column_end: self.column + 1,
            error_type: TokenizerErrorType::InvalidCharacterInString,
        })
    }

    #[inline(always)]
    fn invalid_escaped_character_in_string(&self) -> Result<Option<Token>, TokenizerError> {
        Err(TokenizerError {
            chars: self.char..=self.char,
            lines: self.line..=self.line,
            column_start: self.column,
            column_end: self.column + 1,
            error_type: TokenizerErrorType::InvalidEscapedCharacterInString,
        })
    }
}

impl<T: Iterator<Item = char>> Iterator for JsonTokenizer<T> {
    type Item = Result<Token, TokenizerError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match (self.state)(self) {
                Ok(Some(token)) => {
                    if token.token_type == TokenType::Eof {
                        break None;
                    } else {
                        break Some(Ok(token));
                    }
                }
                Ok(None) => (),
                Err(err) => break Some(Err(err)),
            }
        }
    }
}
