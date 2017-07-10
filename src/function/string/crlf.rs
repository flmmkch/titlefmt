use super::{ Function, Error };
use ::metadata;
use ::expression::{ Expression, Evaluation, Value };

fn crlf<T: metadata::Provider>(expressions: &[Box<Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
    if expressions.len() != 1 {
        return Err(Error::ArgumentError);
    }
    let (original_string, truth) = expect_string_result!(&expressions[0], provider);
    let result_text = original_string + "\n";
    Ok(Evaluation::new(Value::Text(result_text), truth))
}

function_object_maker!(crlf);
