use super::super::*;
use super::super::function::Function;
use super::super::value::{ Evaluation, Value };

fn greater<T: metadata::Provider>(expressions: &[Box<expression::Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
    if expressions.len() != 2 {
        return Err(Error::ArgumentError);
    }
    let mut values : [i32; 2] = [0, 0];
    for i in 0..values.len() {
        let (val, _) = expect_integer_result!(&expressions[i], provider);
        values[i] = val;
    }
    Ok(Evaluation::new(Value::Empty, values[0] > values[1]))
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
