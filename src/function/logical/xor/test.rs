use ::{ test, Formatter };
use std::collections::HashMap;

#[test]
fn test_function_xor()
{
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = {
            let mut dict = HashMap::new();
            dict.insert("title", "Flood");
            dict.insert("artist", "Boris");
            test::MetadataProvider::new(dict)
        };
        {
            let expression = formatter.parser().parse("$if($xor(test, test2, test3), ok, not)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("not", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if($xor(%title%, test2, test3), ok, not)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("ok", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if($xor(%title%, %artist%, test3), ok, not)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("not", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if($xor(%title%, %artist%, %title%. test), ok, not)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("ok", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if($xor(%title%, %album%, test3), ok, not)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("ok", s.to_string().as_str());
        }
    }
}
