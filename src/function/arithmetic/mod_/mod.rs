use super::Error;
use crate::expression::{Evaluation, Expression, Value};
use crate::metadata;

pub fn mod_<T: metadata::Provider>(
    expressions: &[Box<Expression<T>>],
    provider: &T,
) -> Result<Evaluation, Error> {
    if expressions.len() < 2 {
        return Err(Error::ArgumentError);
    }
    // get the first argument
    let (mut result, mut truth) = expect_integer_result!(&expressions[0], provider);
    // use the modulo operator on the subsequent arguments
    for expr in expressions[1..].iter() {
        if let Some((i, expr_truth)) = try_integer_result!(expr, provider) {
            truth |= expr_truth;
            result %= i;
        }
    }
    Ok(Evaluation::new(Value::Integer(result), truth))
}

#[cfg(test)]
mod test;
