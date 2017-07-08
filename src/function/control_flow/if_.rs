use super::super::*;
use super::super::function::Function;
use super::super::value::Value;

fn if_<T: metadata::Provider>(provider: &T, expressions: &[Box<expression::Expression<T>>]) -> Result<Value, Error> {
    match expressions.len() {
        2 | 3 => (),
        _ => return Err(Error::ArgumentError),
    }
    let (expr_value, s) = expressions[0].apply_valued(provider);
    match (expr_value, s) {
        (Value::Empty, _) | (Value::Boolean(false), _) | (Value::Text(_), 0) => {
            match expressions.len() {
                2 => Ok(Value::Boolean(false)),
                3 => Ok(expressions[2].apply(provider)),
                _ => Err(Error::ArgumentError),
            }
        }
        _ => Ok(expressions[1].apply(provider)),
    }
}

pub fn make_function_object<T: metadata::Provider>() -> Function<T> {
    Function::new(
        "if",
        Box::new(|provider: &T, expressions: &[Box<expression::Expression<T>>]| -> Result<Value, Error> { if_(provider, expressions) })
    )
}

#[test]
fn test_function()
{
    let formatter = super::super::Formatter::new();
    // tests with functions
    {
        let test_metadata = {
            let mut dict = HashMap::new();
            dict.insert("tracknumber", "9");
            dict.insert("title", "9th Symphony");
            dict.insert("composer", "Beethoven");
            super::super::tests::MetadataProvider::new(dict)
        };
        {
            let expression = formatter.parser().parse("$if(test, ok, non)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("non", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if(%title%, ok, non)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("ok", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if(%artist%, ok, non)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("non", s.to_string().as_str());
        }
    }
}
