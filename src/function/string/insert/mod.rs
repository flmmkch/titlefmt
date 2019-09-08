use super::Error;
use crate::expression::{Evaluation, Expression, Value};
use crate::metadata;

pub fn insert<T: metadata::Provider>(
    expressions: &[Box<Expression<T>>],
    provider: &T,
) -> Result<Evaluation, Error> {
    if expressions.len() < 3 {
        return Err(Error::ArgumentError);
    }
    let (mut text, truth) = expect_string_result!(&expressions[0], provider);
    let (inserted_string, truth2) = expect_string_result!(&expressions[1], provider);
    let (n_chars, _) = expect_integer_result!(&expressions[2], provider, usize);
    if let Some((n_bytes, _)) = text.char_indices().skip(n_chars).next() {
        text.insert_str(n_bytes, &inserted_string);
        Ok(Evaluation::new(Value::Text(text), truth && truth2))
    } else {
        Err(Error::ArgumentError)
    }
}

#[cfg(test)]
mod test;
