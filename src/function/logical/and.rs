use super::{ Function, Error };
use ::metadata;
use ::expression::{ Expression, Evaluation, Value };

fn and<T: metadata::Provider>(expressions: &[Box<Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
    let result : bool = expressions.iter().all(|ref expr| { expr.apply(provider).truth() });
    Ok(Evaluation::new(Value::Empty, result))
}

function_object_maker!(and);
