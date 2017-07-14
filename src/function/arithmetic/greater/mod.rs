use super::{ Function, Error };
use ::metadata;
use ::expression::{ Expression, Evaluation, Value };

fn greater<T: metadata::Provider>(expressions: &[Box<Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
    if expressions.len() != 2 {
        return Err(Error::ArgumentError);
    }
    let mut values : [i32; 2] = [0, 0];
    for i in 0..values.len() {
        let (val, _) = expect_integer_result!(&expressions[i], provider);
        values[i] = val;
    }
    Ok(Evaluation::new(Value::Empty, values[0] > values[1]))
}

function_object_maker!(greater);

#[cfg(test)]
mod test;
