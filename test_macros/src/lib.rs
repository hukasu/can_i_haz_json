use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn make_test_suite(token_stream: TokenStream) -> TokenStream {
    let mut token_iter = token_stream.into_iter();
    let Some(first_argument) = token_iter.next() else {
        panic!("`make_test_suite` must have 1 argument.");
    };
    if token_iter.next().is_some() {
        panic!("`make_test_suite` must have 1 argument.");
    }

    match first_argument {
        TokenTree::Literal(literal) => {
            let Ok(_current_dir) = std::env::current_dir() else {
                panic!("`make_test_suite` couldn't determinate current dir.");
            };

            let literal_string = literal.to_string();
            let path = literal_string.trim_matches('"');

            match std::fs::exists(&path) {
                Ok(true) => (),
                Ok(false) => panic!("`make_test_suite` must take in a valid path."),
                Err(err) => {
                    panic!(
                        "`make_test_suite` failed to generate test suite due to Io error.\r\n{err}."
                    )
                }
            }

            let mut tests = vec![];
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
    let test_path = "{}";
    let data = std::fs::read(test_path).unwrap();
    let contents = String::from_utf8_lossy(data.as_slice());
    
    BoxJsonDocument::parse(&contents).unwrap();
}}
"#,
                        file_name
                            .to_string_lossy()
                            .trim_end_matches(".json")
                            .replace(".", "_dot_")
                            .replace("-", "_hyphen_")
                            .replace("+", "_plus_")
                            .replace("#", "_pound_"),
                        dir_entry_path.display()
                    );

                    tests.push(test);
                }
            }

            tests.into_iter().collect::<String>().parse().unwrap()
        }
        _ => {
            panic!("`make_test_suite` can only take in String literals.");
        }
    }
}
