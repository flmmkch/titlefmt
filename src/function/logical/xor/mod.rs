use super::{ Function, Error };
use ::metadata;
use ::expression::{ Expression, Evaluation, Value };

fn xor<T: metadata::Provider>(expressions: &[Box<Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
    /// XOR operation:
    /// test if an odd number of arguments evaluate to true
    /// to achieve that: filter out the false expressions and count the remaining (true) expressions
    let result : usize = expressions.iter().filter(|&expr| { expr.apply(provider).truth() }).count();
    let is_odd : bool = result % 2 == 1;
    Ok(Evaluation::new(Value::Empty, is_odd))
}

function_object_maker!(xor);

#[cfg(test)]
mod test;
