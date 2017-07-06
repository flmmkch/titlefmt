use super::*;
use super::value::Value;

fn greater<T: metadata::Provider>(provider: &T, expressions: &[Box<expression::Expression<T>>]) -> Result<Value, Error> {
    if expressions.len() != 2 {
        return Err(Error::ArgumentError);
    }
    let mut values : [i32; 2] = [0, 0];
    for i in 0..2 {
        match expressions[i].apply(provider) {
            Value::Integer(term) => values[i] = term,
            Value::Double(term) => values[i] = term as i32,
            Value::Text(s) => {
                match s.parse::<i32>() {
                    Ok(term) => values[i] = term,
                    _ => return Err(Error::TypeError),
                }
            }
            _ => return Err(Error::TypeError),
        }
    }
    Ok(Value::Boolean(values[0] > values[1]))
}

function_object_maker!(greater);

#[test]
fn test_function()
{
    let formatter = super::super::Formatter::new();
    // tests with functions
    {
        let test_metadata = super::super::tests::MetadataProvider::new(HashMap::new());
        {
            let expression = formatter.parser().parse("$if($greater(7,3), ok, no)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("ok", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if($greater(1,3), ok, no)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("no", s.to_string().as_str());
        }
    }
}
