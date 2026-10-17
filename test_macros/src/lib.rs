use std::{collections::HashMap, path::Path};

use proc_macro::{Literal, TokenStream, TokenTree};
use serde::Deserialize;

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
{}
fn {}() {{
    init_logger();
    let test_path = "{}";
    let contents = std::fs::read_to_string(test_path).unwrap();
    ::can_i_haz_gltf::json::Json::new(::std::borrow::Cow::Owned(contents)).unwrap();
}}
"#,
                if file_name.to_string_lossy().starts_with("n_") {
                    "\n#[should_panic]"
                } else {
                    ""
                },
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

#[proc_macro]
pub fn make_gltf_tests(token_stream: TokenStream) -> TokenStream {
    let literal = take_path(token_stream, "make_gltf_tests");

    let Ok(_current_dir) = std::env::current_dir() else {
        panic!("`make_gltf_tests` couldn't determinate current dir.");
    };

    let literal_string = literal.to_string();
    let path = Path::new(literal_string.trim_matches('"'));

    match std::fs::exists(&path) {
        Ok(true) => (),
        Ok(false) => panic!("`make_gltf_tests` must take in a valid path."),
        Err(err) => {
            panic!("`make_gltf_tests` failed to generate test suite due to Io error.\r\n{err}.")
        }
    }

    let Ok(model_index) = std::fs::File::open(path.join("model-index.json")) else {
        panic!("`make_gltf_tests` failed to read model index.");
    };
    let Ok(gltf_index): Result<Vec<GltfEntry>, serde_json::Error> =
        serde_json::from_reader(model_index)
    else {
        panic!("`make_gltf_tests` failed to parse model index.");
    };

    let mut tests = vec!["".to_owned()];
    for gltf_entry in gltf_index.into_iter() {
        let name = gltf_entry.name;

        for (variant, variant_gltf) in gltf_entry.variants {
            let variant_final_path = path.join(&name).join(&variant).join(variant_gltf);

            let test = format!(
                r#"
#[test]
fn {}_{}() {{
    let test_path = "{}";
    let contents = std::fs::read(test_path).unwrap();
    ::can_i_haz_gltf::Gltf::new(::std::borrow::Cow::Owned(contents)).unwrap();
}}
"#,
                name.replace("2", "Two")
                    .replace(" ", "_")
                    .replace("❤♻", "Emoji"),
                variant.replace("-", "_"),
                variant_final_path.display()
            );

            tests.push(test);
        }
    }

    tests.into_iter().collect::<String>().parse().unwrap()
}

#[derive(Debug, Deserialize)]
struct GltfEntry {
    name: String,
    #[allow(dead_code, reason = "No use for the screenshot path.")]
    screenshot: String,
    variants: HashMap<String, String>,
}
