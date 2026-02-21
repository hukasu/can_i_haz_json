//! Run test suite created by Nicolas Seriot for the
//! [Parsing JSON is a Minefield](https://seriot.ch/software/parsing_json.html) presentation.

use can_i_haz_gltf::json::BoxJsonDocument;

#[test]
fn test_suite() {
    let mut any = false;

    for dir_entry in std::fs::read_dir("test_suite/test_parsing/").unwrap() {
        let dir_entry = dir_entry.unwrap();
        let data = std::fs::read(dir_entry.path()).unwrap();
        let contents = String::from_utf8_lossy(data.as_slice());

        if let Err(err) = BoxJsonDocument::parse(&contents) {
            println!("- {:?} failed with:", dir_entry.path());
            println!("{err}");
            println!();
            any = true;
        }
    }

    if any {
        panic!("A test in the test suite failed.");
    }
}
