use super::{ Function, Error };
use ::metadata;
use ::expression::{ Expression, Evaluation, Value };

fn cut<T: metadata::Provider>(expressions: &[Box<Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
    if expressions.len() != 2 {
        return Err(Error::ArgumentError);
    }
    let (original_string, truth) = expect_string_result!(&expressions[0], provider);
    let (max_len, _) = expect_integer_result!(&expressions[1], provider, usize);
    let result_text : String = original_string.chars().take(max_len).collect();
    Ok(Evaluation::new(Value::Text(result_text), truth))
}

function_object_maker!(cut);
