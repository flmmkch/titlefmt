use super::Error;
use expression::{Evaluation, Expression, Value};
use metadata;

pub fn substr<T: metadata::Provider>(
    expressions: &[Box<Expression<T>>],
    provider: &T,
) -> Result<Evaluation, Error> {
    if expressions.len() < 2 {
        return Err(Error::ArgumentError);
    }
    let (text, truth) = expect_string_result!(&expressions[0], provider);
    let (mut from, _) = expect_integer_result!(&expressions[1], provider, usize);
    if from > 0 {
        from -= 1;
    }
    let (to, _) = expect_integer_result!(&expressions[2], provider, usize);
    let from_bytes_opt = text.char_indices().skip(from).next().map(|i_c| i_c.0);
    let to_bytes_opt = text.char_indices().skip(to).next().map(|i_c| i_c.0);
    match (from_bytes_opt, to_bytes_opt) {
        (Some(from_bytes), Some(to_bytes)) if from_bytes <= to_bytes => {
            let result_text: String = text[from_bytes..to_bytes].to_owned();
            Ok(Evaluation::new(Value::Text(result_text), truth))
        }
        (Some(from_bytes), None) => {
            let result_text: String = text[from_bytes..].to_owned();
            Ok(Evaluation::new(Value::Text(result_text), truth))
        }
        _ => Err(Error::ArgumentError),
    }
}

#[cfg(test)]
mod test;
