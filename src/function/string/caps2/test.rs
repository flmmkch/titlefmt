use ::{ test, Formatter };
use std::collections::HashMap;

#[test]
fn test_function_caps2()
{
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = test::MetadataProvider::new(HashMap::new());
        {
            let expression = formatter.parser().parse("$caps2(ça t''éTonne ね?)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("Ça T'éTonne ね?", s.to_string().as_str());
        }
    }
}
