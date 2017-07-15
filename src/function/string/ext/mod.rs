use super::{ Function, Error };
use ::metadata;
use ::expression::{ Expression, Evaluation, Value };
use std::path::{ Path };
use std::ops::Deref;

fn ext<T: metadata::Provider>(expressions: &[Box<Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
    if expressions.len() < 1 {
        return Err(Error::ArgumentError);
    }
    let (original_string, truth) = expect_string_result!(&expressions[0], provider);
    let file_path = Path::new(original_string.as_str());
    let result_text: String = {
        match file_path.extension() {
            Some(ext) => ext.to_string_lossy().deref().to_owned(),
            None => "".to_owned(),
        }
    };
    Ok(Evaluation::new(Value::Text(result_text), truth))
}

function_object_maker!(ext);

#[cfg(test)]
mod test;
