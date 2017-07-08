use super::*;
use super::value::Value;

fn not<T: metadata::Provider>(provider: &T, expressions: &[Box<expression::Expression<T>>]) -> Result<Value, Error> {
    if expressions.len() !=1 {
        return Err(Error::ArgumentError);
    }
    let result : bool = expect_bool_result(&expressions[0], provider);
    Ok(Value::Boolean(!result))
}

function_object_maker!(not);
