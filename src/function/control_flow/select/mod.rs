use super::{ Function, Error };
use ::metadata;
use ::expression::{ Expression, Evaluation, Value };

fn select<T: metadata::Provider>(expressions: &[Box<Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
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

#[cfg(test)]
mod test;
