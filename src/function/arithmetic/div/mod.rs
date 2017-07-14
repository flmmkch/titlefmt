use super::{ Function, Error };
use ::metadata;
use ::expression::{ Expression, Evaluation, Value };

fn div<T: metadata::Provider>(expressions: &[Box<Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
    if expressions.len() < 2 {
        return Err(Error::ArgumentError);
    }
    // get the first argument
    let (mut result, mut truth) = expect_integer_result!(&expressions[0], provider);
    // divide by the following arguments
    for expr in expressions[1..].iter() {
        if let Some((i, expr_truth)) = try_integer_result!(expr, provider) {
            // division by zero
            if i == 0 {
                return Err(Error::ArgumentError);
            }
            truth |= expr_truth;
            result /= i;
        }
    }
    Ok(Evaluation::new(Value::Integer(result), truth))
}

function_object_maker!(div);

#[cfg(test)]
mod test;
