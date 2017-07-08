use super::super::*;
use super::super::function::Function;
use super::super::value::Value;

fn add<T: metadata::Provider>(provider: &T, expressions: &[Box<expression::Expression<T>>]) -> Result<Value, Error> {
    let mut result : i32 = 0;
    for expr in expressions.iter() {
        match expr.apply(provider) {
            Value::Integer(term) => result += term,
            Value::Double(term) => result += term as i32,
            Value::Text(s) => {
                match s.parse::<i32>() {
                    Ok(term) => result += term,
                    _ => (),
                }
            }
            _ => (),
        }
    }
    Ok(Value::Integer(result))
}

function_object_maker!(add);