use super::{ Function, Error };
use ::metadata;
use ::expression::{ Expression, Evaluation, Value };

fn add<T: metadata::Provider>(expressions: &[Box<Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
    let mut result : i32 = 0;
    let mut truth = false;
    for expr in expressions.iter() {
        if let Some((i, expr_truth)) = try_integer_result!(expr, provider) {
            truth |= expr_truth;
            result += i;
        }
    }
    Ok(Evaluation::new(Value::Integer(result), truth))
}

function_object_maker!(add);

#[cfg(test)]
mod test;
