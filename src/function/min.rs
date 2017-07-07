use super::*;
use super::value::Value;
use std::cmp;

fn min<T: metadata::Provider>(provider: &T, expressions: &[Box<expression::Expression<T>>]) -> Result<Value, Error> {
    if expressions.len() < 1 {
        return Err(Error::ArgumentError);
    }
    // get the first argument
    let mut result = expect_integer_result::<i32, T>(&expressions[0], provider)?;
    for expr in expressions[1..].iter() {
        match expr.apply(provider) {
            Value::Integer(term) => result = cmp::min(result, term),
            Value::Double(term) => result = cmp::min(result, term as i32),
            Value::Text(s) => {
                match s.parse::<i32>() {
                    Ok(term) => result = cmp::min(result, term),
                    _ => (),
                }
            }
            _ => (),
        }
    }
    Ok(Value::Integer(result))
}

function_object_maker!(min);

#[test]
fn test_function()
{
    let formatter = super::super::Formatter::new();
    // tests with functions
    {
        let test_metadata = super::super::tests::MetadataProvider::new(HashMap::new());
        {
            let expression = formatter.parser().parse("$min(7,3)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("3", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$min(3,7)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("3", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$min(7,3,2,5)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("2", s.to_string().as_str());
        }
    }
}
