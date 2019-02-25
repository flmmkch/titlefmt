use std::collections::HashMap;
use {test, Formatter};

#[test]
fn test_function_max() {
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = test::MetadataProvider::new(HashMap::new());
        {
            let expression = formatter.parser().parse("$max(7,3)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("7", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$max(3,7)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("7", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$max(7,3,8,5)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("8", s.to_string().as_str());
        }
    }
}
