use super::super::*;
use super::super::function::Function;
use super::super::value::{ Evaluation, Value };

fn select<T: metadata::Provider>(expressions: &[Box<expression::Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
    if expressions.len() == 0 {
        return Err(Error::ArgumentError);
    }
    let index = {
        let (res_i, _) = expect_integer_result!(&expressions[0], provider, usize);
        res_i - 1
    };
    let value_expressions = &expressions[1..];
    {
        let len = value_expressions.len();
        if index < len {
            Ok(value_expressions[index].apply(provider))
        }
        else {
            Ok(Evaluation::new(Value::Empty, false))
        }
    }
}

function_object_maker!(select);

#[test]
fn test_function()
{
    let formatter = super::super::Formatter::new();
    // tests with functions
    {
        let test_metadata = super::super::tests::MetadataProvider::new(HashMap::new());
        {
            let expression = formatter.parser().parse("$select(2, one, two, three, four)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("two", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$select(4, one, two, three, four)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("four", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$select(5, one, two, three, four)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$select(6, one, two, three, four)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$select(-1, one, two, three, four)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("", s.to_string().as_str());
        }
    }
}
