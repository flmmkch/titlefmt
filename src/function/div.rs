use super::*;
use super::value::Value;

fn div<T: metadata::Provider>(provider: &T, expressions: &[Box<expression::Expression<T>>]) -> Result<Value, Error> {
    if expressions.len() < 2 {
        return Err(Error::ArgumentError);
    }
    // get the first argument
    let mut result : i32 = match expressions[0].apply(provider) {
        Value::Integer(term) => term,
        Value::Double(term) => term as i32,
        Value::Text(s) => {
            match s.parse::<i32>() {
                Ok(term) => term,
                _ => return Err(Error::TypeError),
            }
        }
        _ => return Err(Error::TypeError),
    };
    for expr in expressions[1..].iter() {
        match expr.apply(provider) {
            Value::Integer(term) => result /= term,
            Value::Double(term) => result /= term as i32,
            Value::Text(s) => {
                match s.parse::<i32>() {
                    Ok(term) => result /= term,
                    _ => (),
                }
            }
            _ => (),
        }
    }
    Ok(Value::Integer(result))
}

function_object_maker!(div);

#[test]
fn test_function()
{
    let formatter = super::super::Formatter::new();
    // tests with functions
    {
        let test_metadata = super::super::tests::MetadataProvider::new(HashMap::new());
        {
            let expression = formatter.parser().parse("$div(7,3)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("2", s.to_string().as_str());
        }
    }
}
