use super::Error;
use expression::{Evaluation, Expression, Value};
use metadata;

pub fn select<T: metadata::Provider>(
    expressions: &[Box<Expression<T>>],
    provider: &T,
) -> Result<Evaluation, Error> {
    if expressions.len() == 0 {
        return Err(Error::ArgumentError);
    }
    let (index, _) = expect_integer_result!(&expressions[0], provider, usize);
    let value_expressions = &expressions[1..];
    {
        let len = value_expressions.len();
        if (index > 0) && (index - 1 < len) {
            Ok(value_expressions[index - 1].apply(provider))
        } else {
            Ok(Evaluation::new(Value::Empty, false))
        }
    }
}

#[cfg(test)]
mod test;
