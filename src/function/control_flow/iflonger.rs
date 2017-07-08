use super::super::*;
use super::super::function::Function;
use super::super::value::Value;

fn iflonger<T: metadata::Provider>(provider: &T, expressions: &[Box<expression::Expression<T>>]) -> Result<Value, Error> {
    if expressions.len() != 4 {
        return Err(Error::ArgumentError);
    }
    let string = expect_string_result(&expressions[0], provider);
    let min_len : usize = expect_integer_result::<usize, T>(&expressions[1], provider)?;
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
