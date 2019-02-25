use super::{Error, Function};
use expression::{Evaluation, Expression};
use metadata;

fn if3<T: metadata::Provider>(
    expressions: &[Box<Expression<T>>],
    provider: &T,
) -> Result<Evaluation, Error> {
    if expressions.len() == 0 {
        return Err(Error::ArgumentError);
    }
    for expr in &expressions[..expressions.len() - 1] {
        let eval = expr.apply(provider);
        if eval.truth() {
            return Ok(eval);
        }
    }
    // else
    let else_value = expressions[expressions.len() - 1].apply(provider);
    Ok(else_value)
}

function_object_maker!(if3);

#[cfg(test)]
mod test;
