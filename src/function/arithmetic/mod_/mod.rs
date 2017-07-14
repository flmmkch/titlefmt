use super::{ Function, Error };
use ::metadata;
use ::expression::{ Expression, Evaluation, Value };

fn mod_<T: metadata::Provider>(expressions: &[Box<Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
    if expressions.len() < 2 {
        return Err(Error::ArgumentError);
    }
    // get the first argument
    let (mut result, mut truth) = expect_integer_result!(&expressions[0], provider);
    // use the modulo operator on the subsequent arguments
    for expr in expressions[1..].iter() {
        if let Some((i, expr_truth)) = try_integer_result!(expr, provider) {
            truth |= expr_truth;
            result %= i;
        }
    }
    Ok(Evaluation::new(Value::Integer(result), truth))
}

pub fn make_function_object<T: metadata::Provider>() -> Function<T> {
    Function::new(
        "mod",
        Box::new(|expressions: &[Box<Expression<T>>], provider: &T| -> Result<Evaluation, Error> { mod_(expressions, provider) })
    )
}

#[cfg(test)]
mod test;
