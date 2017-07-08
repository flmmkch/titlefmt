use super::super::*;
use super::super::function::Function;
use super::super::value::Value;

fn or<T: metadata::Provider>(provider: &T, expressions: &[Box<expression::Expression<T>>]) -> Result<Value, Error> {
    let result : bool = expressions.iter().any(|ref expr| { expect_bool_result(&expr, provider) });
    Ok(Value::Boolean(result))
}

function_object_maker!(or);
