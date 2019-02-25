use super::{Error, Function};
use expression::{Evaluation, Expression, Value};
use metadata;

use std::cmp;

fn min<T: metadata::Provider>(
    expressions: &[Box<Expression<T>>],
    provider: &T,
) -> Result<Evaluation, Error> {
    if expressions.len() < 1 {
        return Err(Error::ArgumentError);
    }
    // get the first argument
    let (mut result, mut truth) = expect_integer_result!(&expressions[0], provider);
    for expr in expressions.iter() {
        if let Some((i, expr_truth)) = try_integer_result!(expr, provider) {
            truth |= expr_truth;
            result = cmp::min(result, i);
        }
    }
    Ok(Evaluation::new(Value::Integer(result), truth))
}

function_object_maker!(min);

#[cfg(test)]
mod test;
