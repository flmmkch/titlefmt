use super::Error;
use crate::expression::{Evaluation, Expression, Value};
use crate::metadata;

pub fn or<T: metadata::Provider>(
    expressions: &[Box<Expression<T>>],
    provider: &T,
) -> Result<Evaluation, Error> {
    let result: bool = expressions
        .iter()
        .any(|ref expr| expr.apply(provider).truth());
    Ok(Evaluation::new(Value::Empty, result))
}
