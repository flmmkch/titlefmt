use super::super::*;
use super::super::function::Function;
use super::super::value::Value;

fn and<T: metadata::Provider>(provider: &T, expressions: &[Box<expression::Expression<T>>]) -> Result<Value, Error> {
    let result : bool = expressions.iter().all(|ref expr| { expect_bool_result(&expr, provider) });
    Ok(Value::Boolean(result))
}

function_object_maker!(and);
