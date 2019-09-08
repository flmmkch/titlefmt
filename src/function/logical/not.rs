use super::Error;
use crate::expression::{Evaluation, Expression, Value};
use crate::metadata;

pub fn not<T: metadata::Provider>(
    expressions: &[Box<Expression<T>>],
    provider: &T,
) -> Result<Evaluation, Error> {
    if expressions.len() != 1 {
        return Err(Error::ArgumentError);
    }
    let result = !expressions[0].apply(provider).truth();
    Ok(Evaluation::new(Value::Empty, result))
}
