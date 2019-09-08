extern crate unicode_normalization;
use super::Error;
use crate::expression::{Evaluation, Expression, Value};
use crate::metadata;

pub fn ascii<T: metadata::Provider>(
    expressions: &[Box<Expression<T>>],
    provider: &T,
) -> Result<Evaluation, Error> {
    if expressions.len() < 1 {
        return Err(Error::ArgumentError);
    }
    let (original_string, truth) = expect_string_result!(&expressions[0], provider);
    let text: String = original_string
        .chars()
        .flat_map(|c| {
            let mut normalized_chars = Vec::new();
            unicode_normalization::char::decompose_canonical(c, |n_c| {
                if n_c.is_ascii() {
                    normalized_chars.push(n_c)
                }
            });
            if normalized_chars.is_empty() {
                normalized_chars.push('?');
            }
            normalized_chars
        })
        .collect();
    Ok(Evaluation::new(Value::Text(text), truth))
}

#[cfg(test)]
mod test;
