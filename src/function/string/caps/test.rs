use std::collections::HashMap;
use {test, Formatter};

#[test]
fn test_function_caps() {
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = test::MetadataProvider::new(HashMap::new());
        {
            let expression = formatter.parser().parse("$caps(hello world)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("Hello World", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$caps(ça t''étonne?)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("Ça T'étonne?", s.to_string().as_str());
        }
        {
            let expression = formatter
                .parser()
                .parse("$caps(ÇA T''ÉTONNE ね?)")
                .unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("Ça T'étonne ね?", s.to_string().as_str());
        }
    }
}
