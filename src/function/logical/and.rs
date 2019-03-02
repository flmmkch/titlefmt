use super::Error;
use expression::{Evaluation, Expression, Value};
use metadata;

pub fn and<T: metadata::Provider>(
    expressions: &[Box<Expression<T>>],
    provider: &T,
) -> Result<Evaluation, Error> {
    let result: bool = expressions
        .iter()
        .all(|ref expr| expr.apply(provider).truth());
    Ok(Evaluation::new(Value::Empty, result))
}
