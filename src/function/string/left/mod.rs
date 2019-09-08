use super::Error;
use crate::expression::{Evaluation, Expression, Value};
use crate::metadata;

pub fn left<T: metadata::Provider>(
    expressions: &[Box<Expression<T>>],
    provider: &T,
) -> Result<Evaluation, Error> {
    if expressions.len() < 2 {
        return Err(Error::ArgumentError);
    }
    let (text, truth) = expect_string_result!(&expressions[0], provider);
    let (len, _) = expect_integer_result!(&expressions[1], provider, usize);
    let n_bytes_opt = text.char_indices().skip(len).next().map(|i_c| i_c.0);
    if let Some(n_bytes) = n_bytes_opt {
        let result_text: String = text[..n_bytes].to_owned();
        Ok(Evaluation::new(Value::Text(result_text), truth))
    } else {
        Ok(Evaluation::new(Value::Text(text), truth))
    }
}

#[cfg(test)]
mod test;
