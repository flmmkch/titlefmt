use super::super::*;
use super::super::function::Function;
use super::super::value::Value;

fn greater<T: metadata::Provider>(provider: &T, expressions: &[Box<expression::Expression<T>>]) -> Result<Value, Error> {
    if expressions.len() != 2 {
        return Err(Error::ArgumentError);
    }
    let values : [i32; 2] = [
        expect_integer_result::<i32, T>(&expressions[0], provider)?,
        expect_integer_result::<i32, T>(&expressions[1], provider)?,
    ];
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
