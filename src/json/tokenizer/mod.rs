#![expect(dead_code)]

mod error;
#[cfg(test)]
mod tests;
mod token;

use alloc::vec::Vec;

use crate::str_cursor::StrCursor;

pub use self::{
    error::{TokenizerError, TokenizerErrorType},
    token::{Token, TokenType},
};

#[expect(type_alias_bounds)]
type State<T: Iterator<Item = char>> =
    fn(&mut JsonTokenizer<T>) -> Result<Option<Token>, TokenizerError>;

pub struct JsonTokenizer<T: Iterator<Item = char>> {
    iterator: T,
    cursor: StrCursor,
    token_start_cursor: StrCursor,
    state: State<T>,
    fused: bool,
}

impl<T: Iterator<Item = char>> JsonTokenizer<T> {
    pub fn new(iterator: T) -> Self {
        Self {
            iterator,
            cursor: StrCursor::default(),
            token_start_cursor: StrCursor::default(),
            state: Self::start,
            fused: false,
        }
    }

    /// Consumes the [`JsonTokenizer`] to its end, collecting
    /// all the tokens into a [`Vec`].
    pub fn tokenize(self) -> Result<Vec<Token>, TokenizerError> {
        self.collect()
    }

    /// Processing the string in search of a token
    fn start(&mut self) -> Result<Option<Token>, TokenizerError> {
        if self.fused {
            return self.eof();
        }
        let Some(current_char) = self.iterator.next() else {
            return self.eof();
        };

        let res = match current_char {
            '\u{0020}' | '\u{000D}' | '\u{0009}' => Ok(None),
            '\u{000A}' => Ok(None),
            '{' => Ok(Some(Token {
                start: self.cursor,
                end: self.cursor + current_char,
                token_type: TokenType::LCurly,
            })),
            '}' => Ok(Some(Token {
                start: self.cursor,
                end: self.cursor + current_char,
                token_type: TokenType::RCurly,
            })),
            '[' => Ok(Some(Token {
                start: self.cursor,
                end: self.cursor + current_char,
                token_type: TokenType::LBracket,
            })),
            ']' => Ok(Some(Token {
                start: self.cursor,
                end: self.cursor + current_char,
                token_type: TokenType::RBracket,
            })),
            ',' => Ok(Some(Token {
                start: self.cursor,
                end: self.cursor + current_char,
                token_type: TokenType::Comma,
            })),
            ':' => Ok(Some(Token {
                start: self.cursor,
                end: self.cursor + current_char,
                token_type: TokenType::Colon,
            })),
            '"' => {
                self.token_start_cursor = self.cursor;
                self.state = Self::string;
                Ok(None)
            }
            _ => self.invalid_token(),
        };

        self.cursor += current_char;

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
                    start: self.token_start_cursor,
                    end: self.cursor + current_char,
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

        self.cursor += current_char;

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

        self.cursor += current_char;

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

        self.cursor += current_char;

        res
    }

    #[inline(always)]
    fn eof(&self) -> Result<Option<Token>, TokenizerError> {
        Ok(Some(Token {
            start: self.cursor,
            end: self.cursor,
            token_type: TokenType::Eof,
        }))
    }

    #[inline(always)]
    fn unexpected_eof(&self) -> Result<Option<Token>, TokenizerError> {
        Err(TokenizerError {
            start: self.cursor,
            end: self.cursor,
            error_type: TokenizerErrorType::UnexpectedEof,
        })
    }

    #[inline(always)]
    fn unmatched_curly(&self) -> Result<Option<Token>, TokenizerError> {
        Err(TokenizerError {
            start: self.cursor,
            end: self.cursor,
            error_type: TokenizerErrorType::UnmatchedCurly,
        })
    }

    #[inline(always)]
    fn unmatched_bracket(&self) -> Result<Option<Token>, TokenizerError> {
        Err(TokenizerError {
            start: self.cursor,
            end: self.cursor,
            error_type: TokenizerErrorType::UnmatchedBracket,
        })
    }
    #[inline(always)]
    fn invalid_token(&self) -> Result<Option<Token>, TokenizerError> {
        Err(TokenizerError {
            start: self.cursor,
            end: self.cursor,
            error_type: TokenizerErrorType::InvalidToken,
        })
    }

    #[inline(always)]
    fn invalid_character_in_string(&self) -> Result<Option<Token>, TokenizerError> {
        Err(TokenizerError {
            start: self.cursor,
            end: self.cursor,
            error_type: TokenizerErrorType::InvalidCharacterInString,
        })
    }

    #[inline(always)]
    fn invalid_escaped_character_in_string(&self) -> Result<Option<Token>, TokenizerError> {
        Err(TokenizerError {
            start: self.cursor,
            end: self.cursor,
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
