use super::super::*;
use super::super::function::Function;
use super::super::value::{ Evaluation, Value };
use std::cmp;

fn max<T: metadata::Provider>(expressions: &[Box<expression::Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
    if expressions.len() < 1 {
        return Err(Error::ArgumentError);
    }
    // get the first argument
    let (mut result, mut truth) = expect_integer_result!(&expressions[0], provider);
    for expr in expressions.iter() {
        if let Some((i, expr_truth)) = try_integer_result!(expr, provider) {
            truth |= expr_truth;
            result = cmp::max(result, i);
        }
    }
    Ok(Evaluation::new(Value::Integer(result), truth))
}

function_object_maker!(max);

#[test]
fn test_function()
{
    let formatter = super::super::Formatter::new();
    // tests with functions
    {
        let test_metadata = super::super::tests::MetadataProvider::new(HashMap::new());
        {
            let expression = formatter.parser().parse("$max(7,3)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("7", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$max(3,7)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("7", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$max(7,3,8,5)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("8", s.to_string().as_str());
        }
    }
}
