use super::super::*;
use super::super::function::Function;
use super::super::value::{ Evaluation, Value };

fn cut<T: metadata::Provider>(expressions: &[Box<expression::Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
    if expressions.len() != 2 {
        return Err(Error::ArgumentError);
    }
    let (original_string, truth) = expect_string_result!(&expressions[0], provider);
    let (max_len, _) = expect_integer_result!(&expressions[1], provider, usize);
    let result_text : String = original_string.chars().take(max_len).collect();
    Ok(Evaluation::new(Value::Text(result_text), truth))
}

function_object_maker!(cut);

#[test]
fn test_function()
{
    let formatter = super::super::Formatter::new();
    // tests with functions
    {
        let test_metadata = super::super::tests::MetadataProvider::new(HashMap::new());
        {
            let expression = formatter.parser().parse("$cut(hello, 1)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("h", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("$cut(小さな恋のうた, 3)").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("小さな", s.to_string().as_str());
        }
    }
}
