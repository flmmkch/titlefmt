use crate::{test, Formatter};
use std::collections::HashMap;

#[test]
fn test_function_div() {
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = test::MetadataProvider::new(HashMap::new());
        {
            let expression = formatter.parser().parse("$div(7,3)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("2", s.to_string().as_str());
        }
    }
}
