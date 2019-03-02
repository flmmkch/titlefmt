use super::Error;
use expression::{Evaluation, Expression, Value};
use metadata;

pub fn greater<T: metadata::Provider>(
    expressions: &[Box<Expression<T>>],
    provider: &T,
) -> Result<Evaluation, Error> {
    if expressions.len() != 2 {
        return Err(Error::ArgumentError);
    }
    let mut values: [i32; 2] = [0, 0];
    for i in 0..values.len() {
        let (val, _) = expect_integer_result!(&expressions[i], provider);
        values[i] = val;
    }
    Ok(Evaluation::new(Value::Empty, values[0] > values[1]))
}

#[cfg(test)]
mod test;
