use crate::{test, Formatter};
use std::collections::HashMap;

#[test]
fn test_function_directory() {
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = test::MetadataProvider::new(HashMap::new());
        {
            let expression = formatter
                .parser()
                .parse("$directory('/home/test/Music/hello world.mp3')")
                .unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("Music", s.to_string().as_str());
        }
        {
            let expression = formatter
                .parser()
                .parse("$directory('/home/test/Music/hello world.mp3', 1)")
                .unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("test", s.to_string().as_str());
        }
        {
            let expression = formatter
                .parser()
                .parse("$directory('/home/test/Music/hello world.mp3', 2)")
                .unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("home", s.to_string().as_str());
        }
        {
            let expression = formatter
                .parser()
                .parse("$directory('/home/test/Music/hello world.mp3', 5)")
                .unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("", s.to_string().as_str());
        }
    }
}
