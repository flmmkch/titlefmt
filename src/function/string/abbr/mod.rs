use super::{Error, Function};
use expression::{Evaluation, Expression, Value};
use metadata;

fn abbr<T: metadata::Provider>(
    expressions: &[Box<Expression<T>>],
    provider: &T,
) -> Result<Evaluation, Error> {
    fn abbreviate(original_string: &str) -> String {
        let mut result_text: Vec<char> = Vec::new();
        // strip spaces and parentheses: remember if the last character was one of those
        let mut keep_first_character = true;
        for c in original_string.chars() {
            let is_whitespace = c.is_whitespace() || (c == '(');
            match (keep_first_character, is_whitespace) {
                (true, false) => {
                    result_text.push(c);
                    keep_first_character = false;
                }
                (_, true) => keep_first_character = true,
                _ => continue,
            }
        }
        result_text.into_iter().collect()
    }
    match expressions.len() {
        1 => {
            // return the abbreviation of the evaluated string
            let (original_string, truth) = expect_string_result!(&expressions[0], provider);
            let result_text = abbreviate(&original_string);
            Ok(Evaluation::new(Value::Text(result_text), truth))
        }
        2 => {
            // return the abbreviation of the evaluated string if it is longer than n where n is the second argument
            let (original_string, truth) = expect_string_result!(&expressions[0], provider);
            let (max_len, _) = expect_integer_result!(&expressions[1], provider, usize);
            let result_text: String = {
                if original_string.len() > max_len {
                    abbreviate(&original_string)
                } else {
                    original_string
                }
            };
            Ok(Evaluation::new(Value::Text(result_text), truth))
        }
        _ => Err(Error::ArgumentError),
    }
}

function_object_maker!(abbr);

#[cfg(test)]
mod test;
