use crate::{test, Formatter};
use std::collections::HashMap;

#[test]
fn test_function_ascii() {
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = test::MetadataProvider::new(HashMap::new());
        let do_test = |input, expected| {
            let expression = formatter
                .parser()
                .parse(&format!("$ascii({})", input))
                .unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!(expected, s.to_string().as_str());
        };
        do_test("HELLO WORLD", "HELLO WORLD");
        do_test("01.HELLO WORLD", "01.HELLO WORLD");
        do_test("01. 静かな朝", "01. ????");
        do_test("01. ü ôêèà@ !^", "01. u oeea@ !^");
    }
}
