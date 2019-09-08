use super::Error;
use crate::expression::{Evaluation, Expression, Value};
use crate::metadata;

pub fn add<T: metadata::Provider>(
    expressions: &[Box<Expression<T>>],
    provider: &T,
) -> Result<Evaluation, Error> {
    let mut result: i32 = 0;
    let mut truth = false;
    for expr in expressions.iter() {
        if let Some((i, expr_truth)) = try_integer_result!(expr, provider) {
            truth |= expr_truth;
            result += i;
        }
    }
    Ok(Evaluation::new(Value::Integer(result), truth))
}

#[cfg(test)]
mod test;
