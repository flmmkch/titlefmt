use std::collections::HashMap;
use {test, Formatter};

#[test]
fn test_function_min() {
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = test::MetadataProvider::new(HashMap::new());
        {
            let expression = formatter.parser().parse("$min(7,3)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("3", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$min(3,7)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("3", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$min(7,3,2,5)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("2", s.to_string().as_str());
        }
    }
}
