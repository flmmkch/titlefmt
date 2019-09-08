use super::Error;
use crate::expression::{Evaluation, Expression, Value};
use crate::metadata;

use std::cmp;

pub fn max<T: metadata::Provider>(
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
            result = cmp::max(result, i);
        }
    }
    Ok(Evaluation::new(Value::Integer(result), truth))
}

#[cfg(test)]
mod test;
