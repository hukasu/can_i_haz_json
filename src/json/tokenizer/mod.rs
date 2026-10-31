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
    stored_token: Option<Token>,
    fused: bool,
}

impl<T: Iterator<Item = char>> JsonTokenizer<T> {
    pub fn new(iterator: T) -> Self {
        Self {
            iterator,
            cursor: StrCursor::default(),
            token_start_cursor: StrCursor::default(),
            state: Self::start,
            stored_token: None,
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
            return Ok(Some(self.eof()));
        }
        let Some(current_char) = self.iterator.next() else {
            return Ok(Some(self.eof()));
        };

        let res = match current_char {
            '\u{0020}' | '\u{000A}' | '\u{000D}' | '\u{0009}' => Ok(None),
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
            '0' => {
                self.token_start_cursor = self.cursor;
                self.state = Self::seen_zero;
                Ok(None)
            }
            '1'..='9' | '-' => {
                self.token_start_cursor = self.cursor;
                self.state = Self::number;
                Ok(None)
            }
            '"' => {
                self.token_start_cursor = self.cursor;
                self.state = Self::string;
                Ok(None)
            }
            't' => {
                self.token_start_cursor = self.cursor;
                self.state = Self::seen_t;
                Ok(None)
            }
            'f' => {
                self.token_start_cursor = self.cursor;
                self.state = Self::seen_f;
                Ok(None)
            }
            'n' => {
                self.token_start_cursor = self.cursor;
                self.state = Self::seen_n;
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

    fn seen_zero(&mut self) -> Result<Option<Token>, TokenizerError> {
        let Some(current_char) = self.iterator.next() else {
            self.stored_token = Some(self.eof());
            return Ok(Some(Token {
                start: self.token_start_cursor,
                end: self.cursor,
                token_type: TokenType::Number,
            }));
        };

        let res = match current_char {
            '0'..='9' => self.digit_after_zero(),
            '.' => {
                self.state = Self::seen_fraction;
                Ok(None)
            }
            ',' => {
                self.state = Self::start;
                self.stored_token = Some(Token {
                    start: self.cursor,
                    end: self.cursor + current_char,
                    token_type: TokenType::Comma,
                });
                Ok(Some(Token {
                    start: self.token_start_cursor,
                    end: self.cursor,
                    token_type: TokenType::Number,
                }))
            }
            '\u{0020}' | '\u{000A}' | '\u{000D}' | '\u{0009}' => Ok(Some(Token {
                start: self.token_start_cursor,
                end: self.cursor,
                token_type: TokenType::Number,
            })),
            _ => self.invalid_character_in_string(),
        };

        self.cursor += current_char;

        res
    }

    fn number(&mut self) -> Result<Option<Token>, TokenizerError> {
        let Some(current_char) = self.iterator.next() else {
            self.stored_token = Some(self.eof());
            return Ok(Some(Token {
                start: self.token_start_cursor,
                end: self.cursor,
                token_type: TokenType::Number,
            }));
        };

        let res = match current_char {
            '0'..='9' => Ok(None),
            '.' => {
                self.state = Self::seen_fraction;
                Ok(None)
            }
            ',' => {
                self.state = Self::start;
                self.stored_token = Some(Token {
                    start: self.cursor,
                    end: self.cursor + current_char,
                    token_type: TokenType::Comma,
                });
                Ok(Some(Token {
                    start: self.token_start_cursor,
                    end: self.cursor,
                    token_type: TokenType::Number,
                }))
            }
            '\u{0020}' | '\u{000A}' | '\u{000D}' | '\u{0009}' => Ok(Some(Token {
                start: self.token_start_cursor,
                end: self.cursor,
                token_type: TokenType::Number,
            })),
            _ => self.invalid_digit_in_number(),
        };

        self.cursor += current_char;

        res
    }

    fn seen_fraction(&mut self) -> Result<Option<Token>, TokenizerError> {
        let Some(current_char) = self.iterator.next() else {
            return self.unexpected_eof();
        };

        let res = match current_char {
            '0'..='9' => {
                self.state = Self::fraction;
                Ok(None)
            }
            ',' | 'e' | 'E' | '\u{0020}' | '\u{000A}' | '\u{000D}' | '\u{0009}' => {
                self.no_digits_after_fraction()
            }
            _ => self.invalid_digit_in_number(),
        };

        self.cursor += current_char;

        res
    }

    fn fraction(&mut self) -> Result<Option<Token>, TokenizerError> {
        let Some(current_char) = self.iterator.next() else {
            self.stored_token = Some(self.eof());
            return Ok(Some(Token {
                start: self.token_start_cursor,
                end: self.cursor,
                token_type: TokenType::Number,
            }));
        };

        let res = match current_char {
            '0'..='9' => Ok(None),
            ',' => {
                self.state = Self::start;
                self.stored_token = Some(Token {
                    start: self.cursor,
                    end: self.cursor + current_char,
                    token_type: TokenType::Comma,
                });
                Ok(Some(Token {
                    start: self.token_start_cursor,
                    end: self.cursor,
                    token_type: TokenType::Number,
                }))
            }
            '\u{0020}' | '\u{000A}' | '\u{000D}' | '\u{0009}' => Ok(Some(Token {
                start: self.token_start_cursor,
                end: self.cursor,
                token_type: TokenType::Number,
            })),
            _ => self.invalid_digit_in_number(),
        };

        self.cursor += current_char;

        res
    }

    fn seen_t(&mut self) -> Result<Option<Token>, TokenizerError> {
        let Some(current_char) = self.iterator.next() else {
            return self.unexpected_eof();
        };

        let res = match current_char {
            'r' => {
                self.state = Self::seen_tr;
                Ok(None)
            }
            _ => {
                self.cursor = self.token_start_cursor;
                self.invalid_token()
            }
        };

        self.cursor += current_char;

        res
    }

    fn seen_tr(&mut self) -> Result<Option<Token>, TokenizerError> {
        let Some(current_char) = self.iterator.next() else {
            return self.unexpected_eof();
        };

        let res = match current_char {
            'u' => {
                self.state = Self::seen_tru;
                Ok(None)
            }
            _ => {
                self.cursor = self.token_start_cursor;
                self.invalid_token()
            }
        };

        self.cursor += current_char;

        res
    }

    fn seen_tru(&mut self) -> Result<Option<Token>, TokenizerError> {
        let Some(current_char) = self.iterator.next() else {
            return self.unexpected_eof();
        };

        let res = match current_char {
            'e' => {
                self.state = Self::start;
                Ok(Some(Token {
                    start: self.token_start_cursor,
                    end: self.cursor + current_char,
                    token_type: TokenType::True,
                }))
            }
            _ => {
                self.cursor = self.token_start_cursor;
                self.invalid_token()
            }
        };

        self.cursor += current_char;

        res
    }

    fn seen_f(&mut self) -> Result<Option<Token>, TokenizerError> {
        let Some(current_char) = self.iterator.next() else {
            return self.unexpected_eof();
        };

        let res = match current_char {
            'a' => {
                self.state = Self::seen_fa;
                Ok(None)
            }
            _ => {
                self.cursor = self.token_start_cursor;
                self.invalid_token()
            }
        };

        self.cursor += current_char;

        res
    }

    fn seen_fa(&mut self) -> Result<Option<Token>, TokenizerError> {
        let Some(current_char) = self.iterator.next() else {
            return self.unexpected_eof();
        };

        let res = match current_char {
            'l' => {
                self.state = Self::seen_fal;
                Ok(None)
            }
            _ => {
                self.cursor = self.token_start_cursor;
                self.invalid_token()
            }
        };

        self.cursor += current_char;

        res
    }

    fn seen_fal(&mut self) -> Result<Option<Token>, TokenizerError> {
        let Some(current_char) = self.iterator.next() else {
            return self.unexpected_eof();
        };

        let res = match current_char {
            's' => {
                self.state = Self::seen_fals;
                Ok(None)
            }
            _ => {
                self.cursor = self.token_start_cursor;
                self.invalid_token()
            }
        };

        self.cursor += current_char;

        res
    }

    fn seen_fals(&mut self) -> Result<Option<Token>, TokenizerError> {
        let Some(current_char) = self.iterator.next() else {
            return self.unexpected_eof();
        };

        let res = match current_char {
            'e' => {
                self.state = Self::start;
                Ok(Some(Token {
                    start: self.token_start_cursor,
                    end: self.cursor + current_char,
                    token_type: TokenType::False,
                }))
            }
            _ => {
                self.cursor = self.token_start_cursor;
                self.invalid_token()
            }
        };

        self.cursor += current_char;

        res
    }

    fn seen_n(&mut self) -> Result<Option<Token>, TokenizerError> {
        let Some(current_char) = self.iterator.next() else {
            return self.unexpected_eof();
        };

        let res = match current_char {
            'u' => {
                self.state = Self::seen_nu;
                Ok(None)
            }
            _ => {
                self.cursor = self.token_start_cursor;
                self.invalid_token()
            }
        };

        self.cursor += current_char;

        res
    }

    fn seen_nu(&mut self) -> Result<Option<Token>, TokenizerError> {
        let Some(current_char) = self.iterator.next() else {
            return self.unexpected_eof();
        };

        let res = match current_char {
            'l' => {
                self.state = Self::seen_nul;
                Ok(None)
            }
            _ => {
                self.cursor = self.token_start_cursor;
                self.invalid_token()
            }
        };

        self.cursor += current_char;

        res
    }

    fn seen_nul(&mut self) -> Result<Option<Token>, TokenizerError> {
        let Some(current_char) = self.iterator.next() else {
            return self.unexpected_eof();
        };

        let res = match current_char {
            'l' => {
                self.state = Self::start;
                Ok(Some(Token {
                    start: self.token_start_cursor,
                    end: self.cursor + current_char,
                    token_type: TokenType::Null,
                }))
            }
            _ => {
                self.cursor = self.token_start_cursor;
                self.invalid_token()
            }
        };

        self.cursor += current_char;

        res
    }

    #[inline(always)]
    fn eof(&self) -> Token {
        Token {
            start: self.cursor,
            end: self.cursor,
            token_type: TokenType::Eof,
        }
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
            start: self.token_start_cursor,
            end: self.cursor,
            error_type: TokenizerErrorType::InvalidCharacterInString,
        })
    }

    #[inline(always)]
    fn invalid_escaped_character_in_string(&self) -> Result<Option<Token>, TokenizerError> {
        Err(TokenizerError {
            start: self.token_start_cursor,
            end: self.cursor,
            error_type: TokenizerErrorType::InvalidEscapedCharacterInString,
        })
    }

    #[inline(always)]
    fn invalid_digit_in_number(&self) -> Result<Option<Token>, TokenizerError> {
        Err(TokenizerError {
            start: self.token_start_cursor,
            end: self.cursor,
            error_type: TokenizerErrorType::InvalidDigitInNumber,
        })
    }

    #[inline(always)]
    fn digit_after_zero(&self) -> Result<Option<Token>, TokenizerError> {
        Err(TokenizerError {
            start: self.token_start_cursor,
            end: self.cursor,
            error_type: TokenizerErrorType::DigitAfterZero,
        })
    }

    #[inline(always)]
    fn no_digits_after_fraction(&self) -> Result<Option<Token>, TokenizerError> {
        Err(TokenizerError {
            start: self.token_start_cursor,
            end: self.cursor,
            error_type: TokenizerErrorType::NoDigitAfterFraction,
        })
    }
}

impl<T: Iterator<Item = char>> Iterator for JsonTokenizer<T> {
    type Item = Result<Token, TokenizerError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(token) = self.stored_token.take() {
                break Some(Ok(token));
            }
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
