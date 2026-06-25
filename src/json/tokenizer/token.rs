use crate::str_cursor::StrCursor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub start: StrCursor,
    pub end: StrCursor,
    pub token_type: TokenType,
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
