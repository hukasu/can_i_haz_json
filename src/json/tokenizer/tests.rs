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

#[test]
fn object() {
    //           111111111122222222223
    //  123456789012345678901234567890
    // 1{_
    // 2____"name":_"can_i_haz_gltf",_
    // 3____"dependencies":_["log"]_
    // 4}
    let json = r#"{
    "name": "can_i_haz_gltf",
    "dependencies": ["log"]
}"#;
    let tokens = JsonTokenizer::new(json.chars()).tokenize().unwrap();
    assert_eq!(
        tokens,
        &[
            Token {
                // .{_
                start: StrCursor {
                    byte: 0,
                    line: 1,
                    column: 1
                },
                // {._
                end: StrCursor {
                    byte: 1,
                    line: 1,
                    column: 2
                },
                token_type: TokenType::LCurly
            },
            Token {
                // ____."name":_"can_i_haz_gltf",_
                start: StrCursor {
                    byte: 6,
                    line: 2,
                    column: 5
                },
                // ____"name".:_"can_i_haz_gltf",_
                end: StrCursor {
                    byte: 12,
                    line: 2,
                    column: 11
                },
                token_type: TokenType::String
            },
            Token {
                // ____"name".:_"can_i_haz_gltf",_
                start: StrCursor {
                    byte: 12,
                    line: 2,
                    column: 11
                },
                // ____"name":._"can_i_haz_gltf",_
                end: StrCursor {
                    byte: 13,
                    line: 2,
                    column: 12
                },
                token_type: TokenType::Colon
            },
            Token {
                // ____"name":_."can_i_haz_gltf",_
                start: StrCursor {
                    byte: 14,
                    line: 2,
                    column: 13
                },
                // ____"name":_"can_i_haz_gltf".,_
                end: StrCursor {
                    byte: 30,
                    line: 2,
                    column: 29
                },
                token_type: TokenType::String
            },
            Token {
                // ____"name":_"can_i_haz_gltf".,_
                start: StrCursor {
                    byte: 30,
                    line: 2,
                    column: 29
                },
                // ____"name":_"can_i_haz_gltf",._
                end: StrCursor {
                    byte: 31,
                    line: 2,
                    column: 30
                },
                token_type: TokenType::Comma
            },
            Token {
                // ____."dependencies":_["log"]_
                start: StrCursor {
                    byte: 36,
                    line: 3,
                    column: 5
                },
                // ____"dependencies".:_["log"]_
                end: StrCursor {
                    byte: 50,
                    line: 3,
                    column: 19
                },
                token_type: TokenType::String
            },
            Token {
                // ____"dependencies".:_["log"]_
                start: StrCursor {
                    byte: 50,
                    line: 3,
                    column: 19
                },
                // ____"dependencies":._["log"]_
                end: StrCursor {
                    byte: 51,
                    line: 3,
                    column: 20
                },
                token_type: TokenType::Colon
            },
            Token {
                // ____"dependencies":_.["log"]_
                start: StrCursor {
                    byte: 52,
                    line: 3,
                    column: 21
                },
                // ____"dependencies":_[."log"]_
                end: StrCursor {
                    byte: 53,
                    line: 3,
                    column: 22
                },
                token_type: TokenType::LBracket
            },
            Token {
                // ____"dependencies":_[."log"]_
                start: StrCursor {
                    byte: 53,
                    line: 3,
                    column: 22
                },
                // ____"dependencies":_["log".]_
                end: StrCursor {
                    byte: 58,
                    line: 3,
                    column: 27
                },
                token_type: TokenType::String
            },
            Token {
                // ____"dependencies":_["log".]_
                start: StrCursor {
                    byte: 58,
                    line: 3,
                    column: 27
                },
                // ____"dependencies":_["log"]._
                end: StrCursor {
                    byte: 59,
                    line: 3,
                    column: 28
                },
                token_type: TokenType::RBracket
            },
            Token {
                // .}
                start: StrCursor {
                    byte: 60,
                    line: 4,
                    column: 1
                },
                // }.
                end: StrCursor {
                    byte: 61,
                    line: 4,
                    column: 2
                },
                token_type: TokenType::RCurly
            },
        ]
    );
}
