use super::super::*;
use super::super::function::Function;
use super::super::value::{ Evaluation, Value };

fn if_<T: metadata::Provider>(expressions: &[Box<expression::Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
    match expressions.len() {
        2 | 3 => (),
        _ => return Err(Error::ArgumentError),
    }
    let eval = expressions[0].apply(provider);
    if eval.truth() {
        Ok(expressions[1].apply(provider))
    }
    else {
        match expressions.len() {
            2 => Ok(Evaluation::new(Value::Empty, false)),
            3 => Ok(expressions[2].apply(provider)),
            _ => Err(Error::ArgumentError),
        }
    }
}

pub fn make_function_object<T: metadata::Provider>() -> Function<T> {
    Function::new(
        "if",
        Box::new(|expressions: &[Box<expression::Expression<T>>], provider: &T| -> Result<Evaluation, Error> { if_(expressions, provider) })
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
