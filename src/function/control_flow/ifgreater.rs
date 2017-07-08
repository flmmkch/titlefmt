use super::super::*;
use super::super::function::Function;
use super::super::value::Value;

fn ifgreater<T: metadata::Provider>(provider: &T, expressions: &[Box<expression::Expression<T>>]) -> Result<Value, Error> {
    if expressions.len() != 4 {
        return Err(Error::ArgumentError);
    }
    let values : [i32; 2] = [
        expect_integer_result::<i32, T>(&expressions[0], provider)?,
        expect_integer_result::<i32, T>(&expressions[1], provider)?,
    ];
    let result = {
        if values[0] > values[1] {
            expressions[2].apply(provider)
        }
        else {
            expressions[3].apply(provider)
        }
    };
    Ok(result)
}

function_object_maker!(ifgreater);
