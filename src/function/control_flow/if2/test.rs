use ::{ test, Formatter };
use std::collections::HashMap;

#[test]
fn test_function_if2()
{
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
            let expression = formatter.parser().parse("%tracknumber%. $if2(%composer%, %tracknumber%) - %title%").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("09. Beethoven - 9th Symphony", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("%tracknumber%. $if2(%artist%, %composer%) - %title%").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("09. Beethoven - 9th Symphony", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("%tracknumber%. $if2(%composer%, %artist%) - %title%").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("09. Beethoven - 9th Symphony", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("%tracknumber%. $if2(%albumartist%, %artist%) - %title%").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("09. ? - 9th Symphony", s.to_string().as_str());
        }
    }
}
