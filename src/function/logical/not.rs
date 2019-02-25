use super::{Error, Function};
use expression::{Evaluation, Expression, Value};
use metadata;

fn not<T: metadata::Provider>(
    expressions: &[Box<Expression<T>>],
    provider: &T,
) -> Result<Evaluation, Error> {
    if expressions.len() != 1 {
        return Err(Error::ArgumentError);
    }
    let result = !expressions[0].apply(provider).truth();
    Ok(Evaluation::new(Value::Empty, result))
}

function_object_maker!(not);
