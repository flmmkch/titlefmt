use std::collections::HashMap;
use {test, Formatter};

#[test]
fn test_function_mod() {
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = test::MetadataProvider::new(HashMap::new());
        {
            let expression = formatter.parser().parse("$mod(7,3)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("1", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$mod(15,6)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("3", s.to_string().as_str());
        }
    }
}
