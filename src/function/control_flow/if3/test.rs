use crate::{test, Formatter};
use std::collections::HashMap;

#[test]
fn test_function_if3() {
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
            let expression = formatter
                .parser()
                .parse("$if3(%composer%, %tracknumber%, %title%)")
                .unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("1969", s.to_string().as_str());
        }
        {
            let expression = formatter
                .parser()
                .parse("$if3(%composer%, %title%, %tracknumber%)")
                .unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("1969", s.to_string().as_str());
        }
    }
}
