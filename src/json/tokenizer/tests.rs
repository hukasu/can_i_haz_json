use crate::json::tokenizer::{JsonTokenizer, Token, TokenType};

#[test]
fn empty() {
    let json = "";
    let tokens = JsonTokenizer::new(json.chars()).tokenize().unwrap();
    assert!(tokens.is_empty());
}

#[test]
fn empty_object() {
    let json = "{}";
    let tokens = JsonTokenizer::new(json.chars()).tokenize().unwrap();
    assert_eq!(
        tokens,
        &[
            Token {
                bytes: 0..=0,
                lines: 1..=1,
                column_start: 1,
                column_end: 2,
                token_type: TokenType::LCurly
            },
            Token {
                bytes: 1..=1,
                lines: 1..=1,
                column_start: 2,
                column_end: 3,
                token_type: TokenType::RCurly
            }
        ]
    );
}

#[test]
fn empty_array() {
    let json = "[]";
    let tokens = JsonTokenizer::new(json.chars()).tokenize().unwrap();
    assert_eq!(
        tokens,
        &[
            Token {
                bytes: 0..=0,
                lines: 1..=1,
                column_start: 1,
                column_end: 2,
                token_type: TokenType::LBracket
            },
            Token {
                bytes: 1..=1,
                lines: 1..=1,
                column_start: 2,
                column_end: 3,
                token_type: TokenType::RBracket
            }
        ]
    );
}

#[test]
fn empty_string() {
    let json = r#""""#;
    let tokens = JsonTokenizer::new(json.chars()).tokenize().unwrap();
    assert_eq!(
        tokens,
        &[Token {
            bytes: 0..=1,
            lines: 1..=1,
            column_start: 2,
            column_end: 2,
            token_type: TokenType::String
        },]
    );
    assert_eq!(&json[tokens[0].bytes.clone()], r#""""#);
}

#[test]
fn unicode_string() {
    let json = r#""🙂""#;
    let tokens = JsonTokenizer::new(json.chars()).tokenize().unwrap();
    assert_eq!(
        tokens,
        &[Token {
            bytes: 0..=5,
            lines: 1..=1,
            column_start: 2,
            column_end: 3,
            token_type: TokenType::String
        },]
    );
    assert_eq!(&json[tokens[0].bytes.clone()], r#""🙂""#);
}

#[test]
fn leading_whitespaces() {
    let json = r#"    {}"#;
    let tokens = JsonTokenizer::new(json.chars()).tokenize().unwrap();
    assert_eq!(
        tokens,
        &[
            Token {
                bytes: 4..=4,
                lines: 1..=1,
                column_start: 5,
                column_end: 6,
                token_type: TokenType::LCurly
            },
            Token {
                bytes: 5..=5,
                lines: 1..=1,
                column_start: 6,
                column_end: 7,
                token_type: TokenType::RCurly
            }
        ]
    );

    let json = r#"
   {}"#;
    let tokens = JsonTokenizer::new(json.chars()).tokenize().unwrap();
    assert_eq!(
        tokens,
        &[
            Token {
                bytes: 4..=4,
                lines: 2..=2,
                column_start: 4,
                column_end: 5,
                token_type: TokenType::LCurly
            },
            Token {
                bytes: 5..=5,
                lines: 2..=2,
                column_start: 5,
                column_end: 6,
                token_type: TokenType::RCurly
            }
        ]
    );

    let json = r#"
   {
}"#;
    let tokens = JsonTokenizer::new(json.chars()).tokenize().unwrap();
    assert_eq!(
        tokens,
        &[
            Token {
                bytes: 4..=4,
                lines: 2..=2,
                column_start: 4,
                column_end: 5,
                token_type: TokenType::LCurly
            },
            Token {
                bytes: 6..=6,
                lines: 3..=3,
                column_start: 1,
                column_end: 2,
                token_type: TokenType::RCurly
            }
        ]
    );
}
