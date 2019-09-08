use crate::{test, Formatter};
use std::collections::HashMap;

#[test]
fn test_function_abbr() {
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = test::MetadataProvider::new(HashMap::new());
        {
            let expression = formatter.parser().parse("$abbr(hello world)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("hw", s.to_string().as_str());
        }
        {
            let expression = formatter
                .parser()
                .parse("$abbr('Hunting & Gathering (Cydonia)')")
                .unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("H&GC", s.to_string().as_str());
        }
        {
            let expression = formatter
                .parser()
                .parse("$abbr('21st Century Schizoid Man (including Mirrors)')")
                .unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("2CSMiM", s.to_string().as_str());
        }
    }
}
