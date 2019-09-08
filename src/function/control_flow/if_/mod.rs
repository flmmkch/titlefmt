use super::Error;
use crate::expression::{Evaluation, Expression, Value};
use crate::metadata;

pub fn if_<T: metadata::Provider>(
    expressions: &[Box<Expression<T>>],
    provider: &T,
) -> Result<Evaluation, Error> {
    match expressions.len() {
        2 | 3 => (),
        _ => return Err(Error::ArgumentError),
    }
    let eval = expressions[0].apply(provider);
    if eval.truth() {
        Ok(expressions[1].apply(provider))
    } else {
        match expressions.len() {
            2 => Ok(Evaluation::new(Value::Empty, false)),
            3 => Ok(expressions[2].apply(provider)),
            _ => Err(Error::ArgumentError),
        }
    }
}

#[cfg(test)]
mod test;
