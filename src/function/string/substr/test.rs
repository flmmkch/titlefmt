use std::collections::HashMap;
use {test, Formatter};

#[test]
fn test_function_substr() {
    let formatter = Formatter::new();
    {
        let test_metadata = test::MetadataProvider::new(HashMap::new());
        let do_test = |original, from_char, to_char, expected| {
            let expression = formatter
                .parser()
                .parse(&format!(
                    r#"$substr({},{},{})"#,
                    original, from_char, to_char
                ))
                .unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!(
                expected,
                s.to_string().as_str(),
                "arguments: {}, {}, {}",
                original,
                from_char,
                to_char
            );
        };
        do_test("HELLO TEST", 1, 5, "HELLO");
        do_test("HELLO TEST", 6, 11, " TEST");
        do_test("HELLO TEST", 6, 30, " TEST");
        do_test("HELLO TEST", 0, 5, "HELLO");
        do_test("HELLO TEST", 0, 30, "HELLO TEST");
        do_test("HELLO TEST", 0, 0, "");
        do_test("HELLO TEST", 1, 1, "H");
        do_test("HELLO TEST", 3, 3, "L");
        do_test("HELLO TEST", 13, 13, "");
        do_test("HELLO TEST", 10, 5, "");
        do_test("HELLO TEST", 10, -3, "");
        do_test("今日は", 1, 1, "今");
        do_test("今日は", 2, 2, "日");
        do_test("今日は", 3, 3, "は");
        do_test("今日は", 2, 3, "日は");
        do_test("今日は", 0, 2, "今日");
        do_test("今日は", 1, 2, "今日");
    }
}
