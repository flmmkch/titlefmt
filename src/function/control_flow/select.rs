use super::super::*;
use super::super::function::Function;
use super::super::value::Value;

fn select<T: metadata::Provider>(provider: &T, expressions: &[Box<expression::Expression<T>>]) -> Result<Value, Error> {
    if expressions.len() == 0 {
        return Err(Error::ArgumentError);
    }
    let index : i32 = expect_integer_result::<i32, T>(&expressions[0], provider)?;
    let value_expressions = &expressions[1..];
    {
        let len = value_expressions.len();
        if (index > 0) && ((index as usize) < len) {
            Ok(value_expressions[index as usize].apply(provider))
        }
        else {
            Ok(Value::Boolean(false))
        }
    }
}

function_object_maker!(select);
