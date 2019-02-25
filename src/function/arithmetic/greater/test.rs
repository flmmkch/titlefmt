use std::collections::HashMap;
use {test, Formatter};

#[test]
fn test_function_greater() {
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = test::MetadataProvider::new(HashMap::new());
        {
            let expression = formatter
                .parser()
                .parse("$if($greater(7,3), ok, no)")
                .unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("ok", s.to_string().as_str());
        }
        {
            let expression = formatter
                .parser()
                .parse("$if($greater(1,3), ok, no)")
                .unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("no", s.to_string().as_str());
        }
    }
}
