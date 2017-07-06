use super::*;
use super::value::Value;

fn sub<T: metadata::Provider>(provider: &T, expressions: &[Box<expression::Expression<T>>]) -> Result<Value, Error> {
    if expressions.len() < 2 {
        return Err(Error::ArgumentError);
    }
    // get the first argument
    let mut result : i32 = expect_result::<i32, T>(&expressions[0], provider)?;
    for expr in expressions[1..].iter() {
        match expr.apply(provider) {
            Value::Integer(term) => result -= term,
            Value::Double(term) => result -= term as i32,
            Value::Text(s) => {
                match s.parse::<i32>() {
                    Ok(term) => result -= term,
                    _ => (),
                }
            }
            _ => (),
        }
    }
    Ok(Value::Integer(result))
}

function_object_maker!(sub);
