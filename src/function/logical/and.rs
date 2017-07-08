use super::super::*;
use super::super::function::Function;
use super::super::value::{ Evaluation, Value };

fn and<T: metadata::Provider>(expressions: &[Box<expression::Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
    let result : bool = expressions.iter().all(|ref expr| { expr.apply(provider).truth() });
    Ok(Evaluation::new(Value::Empty, result))
}

function_object_maker!(and);
