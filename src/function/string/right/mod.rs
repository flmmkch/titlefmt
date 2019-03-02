use super::Error;
use expression::{Evaluation, Expression, Value};
use metadata;

pub fn right<T: metadata::Provider>(
    expressions: &[Box<Expression<T>>],
    provider: &T,
) -> Result<Evaluation, Error> {
    if expressions.len() < 2 {
        return Err(Error::ArgumentError);
    }
    let (text, truth) = expect_string_result!(&expressions[0], provider);
    let (len, _) = expect_integer_result!(&expressions[1], provider, usize);
    match len {
        // only if len > 0: right(_, 0) is empty so don't take the rightmost character
        len if len > 0 => {
            // shift by -1
            let n_bytes_opt = text
                .char_indices()
                .rev()
                .skip(len - 1)
                .next()
                .map(|i_c| i_c.0);
            if let Some(n_bytes) = n_bytes_opt {
                let result_text: String = text[n_bytes..].to_owned();
                Ok(Evaluation::new(Value::Text(result_text), truth))
            } else {
                Ok(Evaluation::new(Value::Text(text), truth))
            }
        }
        _ => Err(Error::ArgumentError),
    }
}

#[cfg(test)]
mod test;
