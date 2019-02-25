use std::collections::HashMap;
use {test, Formatter};

#[test]
fn test_function_select() {
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = test::MetadataProvider::new(HashMap::new());
        {
            let expression = formatter
                .parser()
                .parse("$select(2, one, two, three, four)")
                .unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("two", s.to_string().as_str());
        }
        {
            let expression = formatter
                .parser()
                .parse("$select(4, one, two, three, four)")
                .unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("four", s.to_string().as_str());
        }
        {
            let expression = formatter
                .parser()
                .parse("$select(5, one, two, three, four)")
                .unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("", s.to_string().as_str());
        }
        {
            let expression = formatter
                .parser()
                .parse("$select(6, one, two, three, four)")
                .unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("", s.to_string().as_str());
        }
        {
            let expression = formatter
                .parser()
                .parse("$select(-1, one, two, three, four)")
                .unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("", s.to_string().as_str());
        }
    }
}
