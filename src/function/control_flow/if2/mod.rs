use super::Error;
use expression::{Evaluation, Expression};
use metadata;

pub fn if2<T: metadata::Provider>(
    expressions: &[Box<Expression<T>>],
    provider: &T,
) -> Result<Evaluation, Error> {
    if expressions.len() != 2 {
        return Err(Error::ArgumentError);
    }
    let eval = expressions[0].apply(provider);
    if eval.truth() {
        Ok(eval)
    } else {
        Ok(expressions[1].apply(provider))
    }
}

#[cfg(test)]
mod test;
