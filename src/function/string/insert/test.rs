use crate::{test, Formatter};
use std::collections::HashMap;

#[test]
fn test_function_insert() {
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = test::MetadataProvider::new(HashMap::new());
        let do_test = |original, inserted, at_char, expected| {
            let expression = formatter
                .parser()
                .parse(&format!(
                    r#"$insert({}, {}, {})"#,
                    original, inserted, at_char
                ))
                .unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!(expected, s.to_string().as_str());
        };
        do_test(
            "HELLO TEST",
            "THIS IS JUST A ",
            6,
            "HELLO THIS IS JUST A TEST",
        );
        do_test("HELLO TEST", "THIS IS JUST A ", 30, "");
        do_test("HELLO TEST", "THIS IS JUST A ", -16, "");
        do_test("今日はさよなら", "今晩は", 3, "今日は今晩はさよなら");
    }
}
