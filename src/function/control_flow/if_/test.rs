use crate::{test, Formatter};
use std::collections::HashMap;

#[test]
fn test_function_if() {
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = {
            let mut dict = HashMap::new();
            dict.insert("tracknumber", "9");
            dict.insert("title", "9th Symphony");
            dict.insert("composer", "Beethoven");
            test::MetadataProvider::new(dict)
        };
        {
            let expression = formatter.parser().parse("$if(test, ok, non)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("non", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if(%title%, ok, non)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("ok", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if(%artist%, ok, non)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("non", s.to_string().as_str());
        }
    }
}
