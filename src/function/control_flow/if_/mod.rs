use super::{ Function, Error };
use ::metadata;
use ::expression::{ Expression, Evaluation, Value };

fn if_<T: metadata::Provider>(expressions: &[Box<Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
    match expressions.len() {
        2 | 3 => (),
        _ => return Err(Error::ArgumentError),
    }
    let eval = expressions[0].apply(provider);
    if eval.truth() {
        Ok(expressions[1].apply(provider))
    }
    else {
        match expressions.len() {
            2 => Ok(Evaluation::new(Value::Empty, false)),
            3 => Ok(expressions[2].apply(provider)),
            _ => Err(Error::ArgumentError),
        }
    }
}

pub fn make_function_object<T: metadata::Provider>() -> Function<T> {
    Function::new(
        "if",
        Box::new(|expressions: &[Box<Expression<T>>], provider: &T| -> Result<Evaluation, Error> { if_(expressions, provider) })
    )
}

#[cfg(test)]
mod test;
