use super::{ Function, Error };
use ::metadata;
use ::expression::{ Expression, Evaluation, Value };

fn iflonger<T: metadata::Provider>(expressions: &[Box<Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
    if expressions.len() != 4 {
        return Err(Error::ArgumentError);
    }
    let (string, _) = expect_string_result!(&expressions[0], provider);
    let (min_len, _) = expect_integer_result!(&expressions[1], provider, usize);
    let result = {
        if string.len() > min_len {
            expressions[2].apply(provider)
        }
        else {
            expressions[3].apply(provider)
        }
    };
    Ok(result)
}

function_object_maker!(iflonger);
