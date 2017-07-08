use super::super::*;
use super::super::function::Function;
use super::super::value::{ Evaluation };

fn if3<T: metadata::Provider>(expressions: &[Box<expression::Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
    if expressions.len() == 0 {
        return Err(Error::ArgumentError);
    }
    for expr in &expressions[..expressions.len() - 1] {
        let eval = expr.apply(provider);
        if eval.truth() {
            return Ok(eval);
        }
    }
    // else
    let else_value = expressions[expressions.len() - 1].apply(provider); 
    Ok(else_value)
}

function_object_maker!(if3);

#[test]
fn test_function()
{
    let formatter = super::super::Formatter::new();
    // tests with functions
    {
        let test_metadata = {
            let mut dict = HashMap::new();
            dict.insert("title", "1969");
            dict.insert("date", "2017");
            dict.insert("artist", "Ulver");
            super::super::tests::MetadataProvider::new(dict)
        };
        {
            let expression = formatter.parser().parse("$if3(%composer%, %tracknumber%, %title%)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("1969", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$if3(%composer%, %title%, %tracknumber%)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("1969", s.to_string().as_str());
        }
    }
}
