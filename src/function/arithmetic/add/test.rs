use ::{ test, Formatter };
use std::collections::HashMap;

#[test]
fn test_function_add()
{
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = {
            let mut dict = HashMap::new();
            dict.insert("title", "1969");
            dict.insert("date", "2017");
            dict.insert("artist", "Ulver");
            test::MetadataProvider::new(dict)
        };
        {
            let expression = formatter.parser().parse("$add(2,3)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("5", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$add(2,3,4,5)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("14", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$add(2,3,-4,5)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("6", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if($add(2,3),ok,no)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("no", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$add(%date%,1)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("2018", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$add(%date%,-1)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("2016", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if($add(%title%,1),ok,no)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("ok", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if($greater($add(%title%,1),1969),ok,no)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("ok", s.to_string().as_str());
        }
    }
}