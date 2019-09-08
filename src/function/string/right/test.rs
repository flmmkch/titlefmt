use crate::{test, Formatter};
use std::collections::HashMap;

#[test]
fn test_function_right() {
    let formatter = Formatter::new();
    {
        let test_metadata = test::MetadataProvider::new(HashMap::new());
        let do_test = |original, at_char, expected| {
            let expression = formatter
                .parser()
                .parse(&format!(r#"$right({}, {})"#, original, at_char))
                .unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!(
                expected,
                s.to_string().as_str(),
                "arguments: {}, {}",
                original,
                at_char
            );
        };
        do_test("HELLO TEST", 4, "TEST");
        do_test("HELLO TEST", 5, " TEST");
        do_test("HELLO TEST", 0, "");
        do_test("HELLO TEST", 30, "HELLO TEST");
        do_test("今日は", 1, "は");
        do_test("今日は", 2, "日は");
        do_test("今日は", 3, "今日は");
    }
}
