use super::super::*;
use super::super::function::Function;
use super::super::value::{ Evaluation, Value };

fn not<T: metadata::Provider>(expressions: &[Box<expression::Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
    if expressions.len() != 1 {
        return Err(Error::ArgumentError);
    }
    let result = !expressions[0].apply(provider).truth();
    Ok(Evaluation::new(Value::Empty, result))
}

function_object_maker!(not);
