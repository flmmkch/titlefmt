use crate::{test, Formatter};
use std::collections::HashMap;

#[test]
fn test_function_hex() {
    let formatter = Formatter::new();
    // tests with functions
    {
        let test_metadata = test::MetadataProvider::new(HashMap::new());
        let do_test = |input, expected| {
            let expression = formatter
                .parser()
                .parse(&format!("$hex({})", input))
                .unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!(expected, s.to_string().as_str());
        };
        do_test("0", "0");
        do_test("1", "1");
        do_test("2", "2");
        do_test("9", "9");
        do_test("10", "A");
        do_test("11", "B");
        do_test("12", "C");
        do_test("13", "D");
        do_test("14", "E");
        do_test("15", "F");
        do_test("16", "10");
        do_test("20", "14");
        do_test("30", "1E");
        do_test("BONJOUR", "");
        do_test("hello this is just a test", "");
        do_test("1E", "");
        do_test("EE", "");
        let do_test2 = |input, padding, expected| {
            let expression = formatter
                .parser()
                .parse(&format!("$hex({}, {})", input, padding))
                .unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!(expected, s.to_string().as_str());
        };
        do_test2("20", 0, "14");
        do_test2("20", 1, "14");
        do_test2("20", 2, "14");
        do_test2("20", 3, "014");
        do_test2("20", 4, "0014");
        do_test2("20", 5, "00014");
        do_test2("13", 1, "D");
        do_test2("13", 3, "00D");
    }
}
