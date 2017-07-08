use super::super::*;
use super::super::function::Function;
use super::super::value::{ Evaluation, Value };

fn or<T: metadata::Provider>(expressions: &[Box<expression::Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
    let result : bool = expressions.iter().any(|ref expr| { expr.apply(provider).truth() });
    Ok(Evaluation::new(Value::Empty, result))
}

function_object_maker!(or);
