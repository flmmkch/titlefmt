use std::collections::HashMap;
use {test, Formatter};

#[test]
fn test_function_directory_path() {
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = test::MetadataProvider::new(HashMap::new());
        {
            let expression = formatter
                .parser()
                .parse("$directory_path('/home/test/Music/hello world.mp3')")
                .unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("/home/test/Music", s.to_string().as_str());
        }
    }
}
