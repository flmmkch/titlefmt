use super::super::*;
use super::super::function::Function;
use super::super::value::Value;

fn xor<T: metadata::Provider>(provider: &T, expressions: &[Box<expression::Expression<T>>]) -> Result<Value, Error> {
    /// XOR operation:
    /// test if an odd number of arguments evaluate to true
    /// to achieve that: filter out the false expressions and count the remaining (true) expressions
    let result : usize = expressions.iter().filter(|&expr| { expect_bool_result(&expr, provider) }).count();
    let is_odd : bool = result % 2 == 1;
    Ok(Value::Boolean(is_odd))
}

function_object_maker!(xor);

#[test]
fn test_function()
{
    let formatter = super::super::Formatter::new();
    // tests with functions
    {
        let test_metadata = {
            let mut dict = HashMap::new();
            dict.insert("title", "Flood");
            dict.insert("artist", "Boris");
            super::super::tests::MetadataProvider::new(dict)
        };
        {
            let expression = formatter.parser().parse("$if($xor(test, test2, test3), ok, not)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("not", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if($xor(%title%, test2, test3), ok, not)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("ok", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if($xor(%title%, %artist%, test3), ok, not)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("not", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if($xor(%title%, %artist%, %title%. test), ok, not)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("ok", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if($xor(%title%, %album%, test3), ok, not)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("ok", s.to_string().as_str());
        }
    }
}
