use super::super::*;
use super::super::function::Function;
use super::super::value::{ Evaluation, Value };

fn mod_<T: metadata::Provider>(expressions: &[Box<expression::Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
    if expressions.len() < 2 {
        return Err(Error::ArgumentError);
    }
    // get the first argument
    let (mut result, mut truth) = expect_integer_result!(&expressions[0], provider);
    // use the modulo operator on the subsequent arguments
    for expr in expressions[1..].iter() {
        if let Some((i, expr_truth)) = try_integer_result!(expr, provider) {
            truth |= expr_truth;
            result %= i;
        }
    }
    Ok(Evaluation::new(Value::Integer(result), truth))
}

pub fn make_function_object<T: metadata::Provider>() -> Function<T> {
    Function::new(
        "mod",
        Box::new(|expressions: &[Box<expression::Expression<T>>], provider: &T| -> Result<Evaluation, Error> { mod_(expressions, provider) })
    )
}

#[test]
fn test_function()
{
    let formatter = super::super::Formatter::new();
    // tests with functions
    {
        let test_metadata = super::super::tests::MetadataProvider::new(HashMap::new());
        {
            let expression = formatter.parser().parse("$mod(7,3)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("1", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$mod(15,6)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("3", s.to_string().as_str());
        }
    }
}
