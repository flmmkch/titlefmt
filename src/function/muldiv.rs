use super::*;
use super::value::Value;

fn muldiv<T: metadata::Provider>(provider: &T, expressions: &[Box<expression::Expression<T>>]) -> Result<Value, Error> {
    if expressions.len() != 3 {
        return Err(Error::ArgumentError);
    }
    // get the first argument
    let a = expect_integer_result::<i32, T>(&expressions[0], provider)?;
    let b = expect_integer_result::<i32, T>(&expressions[1], provider)?;
    let c = expect_integer_result::<i32, T>(&expressions[2], provider)?;
    Ok(Value::Integer((a * b) / c))
}

function_object_maker!(muldiv);
