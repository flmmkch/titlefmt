use ::{ test, Formatter };
use std::collections::HashMap;

#[test]
fn test_function_cut()
{
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = test::MetadataProvider::new(HashMap::new());
        {
            let expression = formatter.parser().parse("$cut(hello, 1)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("h", s.to_string().as_str());
        }
    }
}
