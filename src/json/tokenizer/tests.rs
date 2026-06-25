use crate::{
    json::tokenizer::{JsonTokenizer, Token, TokenType},
    str_cursor::StrCursor,
};

#[test]
fn empty() {
    let json = "";
    let tokens = JsonTokenizer::new(json.chars()).tokenize().unwrap();
    assert!(tokens.is_empty());
}

#[test]
fn empty_object() {
    //  12
    // 1{}
    let json = "{}";
    let tokens = JsonTokenizer::new(json.chars()).tokenize().unwrap();
    assert_eq!(
        tokens,
        &[
            Token {
                // .{}
                start: StrCursor {
                    byte: 0,
                    line: 1,
                    column: 1,
                },
                // {.}
                end: StrCursor {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                token_type: TokenType::LCurly
            },
            Token {
                // {.}
                start: StrCursor {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                // {}.
                end: StrCursor {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                token_type: TokenType::RCurly
            }
        ]
    );
}

#[test]
fn empty_array() {
    //  12
    // 1[]
    let json = "[]";
    let tokens = JsonTokenizer::new(json.chars()).tokenize().unwrap();
    assert_eq!(
        tokens,
        &[
            Token {
                // .[]
                start: StrCursor {
                    byte: 0,
                    line: 1,
                    column: 1,
                },
                // [.]
                end: StrCursor {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                token_type: TokenType::LBracket
            },
            Token {
                // [.]
                start: StrCursor {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                // [].
                end: StrCursor {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                token_type: TokenType::RBracket
            }
        ]
    );
}

#[test]
fn empty_string() {
    //  12
    // 1""
    let json = r#""""#;
    let tokens = JsonTokenizer::new(json.chars()).tokenize().unwrap();
    assert_eq!(
        tokens,
        &[Token {
            // .""
            start: StrCursor {
                byte: 0,
                line: 1,
                column: 1,
            },
            // "".
            end: StrCursor {
                byte: 2,
                line: 1,
                column: 3,
            },
            token_type: TokenType::String
        },]
    );
    let head = &tokens[0];
    assert_eq!(&json[head.start.to_range(&head.end)], r#""""#);
}

#[test]
fn unicode_string() {
    //  12 3
    // 1"🙂"
    let json = r#""🙂""#;
    let tokens = JsonTokenizer::new(json.chars()).tokenize().unwrap();
    const LENGTH_OF_EMOJI: usize = '🙂'.len_utf8();
    assert_eq!(
        tokens,
        &[Token {
            // ."🙂"
            start: StrCursor {
                byte: 0,
                line: 1,
                column: 1
            },
            // "🙂".
            end: StrCursor {
                byte: 2 + LENGTH_OF_EMOJI,
                line: 1,
                column: 4
            },
            token_type: TokenType::String
        },]
    );
    let head = &tokens[0];
    assert_eq!(&json[head.start.to_range(&head.end)], r#""🙂""#);
}

#[test]
fn leading_whitespaces() {
    //  123456
    // 1____{}
    let json = r#"    {}"#;
    let tokens = JsonTokenizer::new(json.chars()).tokenize().unwrap();
    assert_eq!(
        tokens,
        &[
            Token {
                // ____.{}
                start: StrCursor {
                    byte: 4,
                    line: 1,
                    column: 5
                },
                // ____{.}
                end: StrCursor {
                    byte: 5,
                    line: 1,
                    column: 6
                },
                token_type: TokenType::LCurly
            },
            Token {
                // ____{.}
                start: StrCursor {
                    byte: 5,
                    line: 1,
                    column: 6
                },
                // ____{}.
                end: StrCursor {
                    byte: 6,
                    line: 1,
                    column: 7
                },
                token_type: TokenType::RCurly
            }
        ]
    );
}

#[test]
fn leading_newline() {
    //  12
    // 1_
    // 2{}
    let json = r#"
{}"#;
    let tokens = JsonTokenizer::new(json.chars()).tokenize().unwrap();
    assert_eq!(
        tokens,
        &[
            Token {
                // _
                // .{}
                start: StrCursor {
                    byte: 1,
                    line: 2,
                    column: 1
                },
                // _
                // {.}
                end: StrCursor {
                    byte: 2,
                    line: 2,
                    column: 2
                },
                token_type: TokenType::LCurly
            },
            Token {
                // _
                // {.}
                start: StrCursor {
                    byte: 2,
                    line: 2,
                    column: 2
                },
                // _
                // {}.
                end: StrCursor {
                    byte: 3,
                    line: 2,
                    column: 3
                },
                token_type: TokenType::RCurly
            }
        ]
    );
}

#[test]
fn multiline_empty_object() {
    //  12
    // 1{_
    // 2}
    let json = r#"{
}"#;
    let tokens = JsonTokenizer::new(json.chars()).tokenize().unwrap();
    assert_eq!(
        tokens,
        &[
            Token {
                // .{_
                // }
                start: StrCursor {
                    byte: 0,
                    line: 1,
                    column: 1
                },
                // {._
                // }
                end: StrCursor {
                    byte: 1,
                    line: 1,
                    column: 2
                },
                token_type: TokenType::LCurly
            },
            Token {
                // {_
                // .}
                start: StrCursor {
                    byte: 2,
                    line: 2,
                    column: 1
                },
                // {_
                // }.
                end: StrCursor {
                    byte: 3,
                    line: 2,
                    column: 2
                },
                token_type: TokenType::RCurly
            },
        ]
    );
}
