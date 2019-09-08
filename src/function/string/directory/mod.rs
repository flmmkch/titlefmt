use super::Error;
use crate::expression::{Evaluation, Expression, Value};
use crate::metadata;
use std::ops::Deref;
use std::path::{Path, PathBuf};

pub fn directory<T: metadata::Provider>(
    expressions: &[Box<Expression<T>>],
    provider: &T,
) -> Result<Evaluation, Error> {
    let go_up = {
        match expressions.len() {
            1 => 0,
            2 => {
                let (go_up, _) = expect_integer_result!(&expressions[1], provider, usize);
                go_up
            }
            _ => return Err(Error::ArgumentError),
        }
    };
    let (original_string, truth) = expect_string_result!(&expressions[0], provider);
    let file_path = Path::new(original_string.as_str());
    let mut result_path: PathBuf = {
        if file_path.is_dir() {
            file_path.to_path_buf()
        } else {
            match file_path.parent() {
                Some(dir) => dir.to_path_buf(),
                None => Path::new("/").to_path_buf(),
            }
        }
    };
    for _ in 0..go_up {
        result_path = {
            match result_path.parent() {
                Some(dir) => dir.to_path_buf(),
                None => Path::new("/").to_path_buf(),
            }
        };
    }
    let result_text: String = {
        match result_path.file_stem() {
            Some(os_str) => os_str.to_string_lossy().deref().to_owned(),
            None => "".to_owned(),
        }
    };
    Ok(Evaluation::new(Value::Text(result_text), truth))
}

#[cfg(test)]
mod test;
