use proc_macro::{Literal, TokenStream, TokenTree};

fn take_path(token_stream: TokenStream, macro_name: &'static str) -> Literal {
    let mut token_iter = token_stream.into_iter();
    let Some(first_argument) = token_iter.next() else {
        panic!("`{macro_name}` must have 1 argument.");
    };
    if token_iter.next().is_some() {
        panic!("`{macro_name}` must have 1 argument.");
    }

    match first_argument {
        TokenTree::Literal(literal) => literal,
        _ => {
            panic!("`{macro_name}` can only take in String literals.",);
        }
    }
}

#[proc_macro]
pub fn make_test_suite(token_stream: TokenStream) -> TokenStream {
    let literal = take_path(token_stream, "make_test_suite");

    let Ok(_current_dir) = std::env::current_dir() else {
        panic!("`make_test_suite` couldn't determinate current dir.");
    };

    let literal_string = literal.to_string();
    let path = literal_string.trim_matches('"');

    match std::fs::exists(&path) {
        Ok(true) => (),
        Ok(false) => panic!("`make_test_suite` must take in a valid path."),
        Err(err) => {
            panic!("`make_test_suite` failed to generate test suite due to Io error.\r\n{err}.")
        }
    }

    let mut tests = vec![
        r#"
// Ensure the logger initializes exactly once across parallel tests
static INIT: ::std::sync::Once = ::std::sync::Once::new();

fn init_logger() {
    INIT.call_once(|| {
        ::env_logger::builder()
            .is_test(true) // Crucial for output capturing
            .try_init()
            .ok();
    });
}
"#
        .to_owned(),
    ];
    for dir_entry in std::fs::read_dir(path).unwrap() {
        let Ok(dir_entry) = dir_entry else {
            panic!("`make_test_suite` failed to read directory entry.");
        };
        let dir_entry_path = dir_entry.path();
        if let Some(file_name) = dir_entry_path.file_name() {
            let test = format!(
                r#"
#[test]
fn {}() {{
    init_logger();
    let test_path = "{}";
    let contents = std::fs::read_to_string(test_path).unwrap();
    let res = ::can_i_haz_json::Json::new(::std::borrow::Cow::Owned(contents));
    if {} {{
        assert!(res.is_err());
    }} else {{
        assert!(res.is_ok());
    }}
}}
"#,
                file_name
                    .to_string_lossy()
                    .trim_end_matches(".json")
                    .replace(".", "_dot_")
                    .replace("-", "_hyphen_")
                    .replace("+", "_plus_")
                    .replace("#", "_pound_"),
                dir_entry_path.display(),
                file_name.to_string_lossy().starts_with("n_"),
            );

            tests.push(test);
        }
    }

    tests.into_iter().collect::<String>().parse().unwrap()
}
