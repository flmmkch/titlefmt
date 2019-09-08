use crate::{test, Formatter};
use std::collections::HashMap;

#[test]
fn test_function_left_cut() {
    let formatter = Formatter::new();
    {
        let test_metadata = test::MetadataProvider::new(HashMap::new());
        let do_test = |original, at_char, expected| {
            for func_name in ["left", "cut"].iter() {
                let expression = formatter
                    .parser()
                    .parse(&format!(r#"${}({}, {})"#, func_name, original, at_char))
                    .unwrap();
                let s = expression.apply(&test_metadata);
                assert_eq!(expected, s.to_string().as_str());
            }
        };
        do_test("HELLO TEST", 5, "HELLO");
        do_test("HELLO TEST", 6, "HELLO ");
        do_test("HELLO TEST", 0, "");
        do_test("HELLO TEST", 30, "HELLO TEST");
        do_test("今日は", 1, "今");
        do_test("今日は", 2, "今日");
        do_test("今日は", 3, "今日は");
    }
}
