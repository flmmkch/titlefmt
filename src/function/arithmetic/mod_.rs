use super::super::*;
use super::super::function::Function;
use super::super::value::Value;

fn mod_<T: metadata::Provider>(provider: &T, expressions: &[Box<expression::Expression<T>>]) -> Result<Value, Error> {
    if expressions.len() < 2 {
        return Err(Error::ArgumentError);
    }
    // get the first argument
    let mut result = expect_integer_result::<i32, T>(&expressions[0], provider)?;
    for expr in expressions[1..].iter() {
        match expr.apply(provider) {
            Value::Integer(term) => result %= term,
            Value::Double(term) => result %= term as i32,
            Value::Text(s) => {
                match s.parse::<i32>() {
                    Ok(term) => result %= term,
                    _ => (),
                }
            }
            _ => (),
        }
    }
    Ok(Value::Integer(result))
}

pub fn make_function_object<T: metadata::Provider>() -> Function<T> {
    Function::new(
        "mod",
        Box::new(|provider: &T, expressions: &[Box<expression::Expression<T>>]| -> Result<Value, Error> { mod_(provider, expressions) })
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
