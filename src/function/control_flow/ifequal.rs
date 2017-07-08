use super::super::*;
use super::super::function::Function;
use super::super::value::{ Evaluation, Value };

fn ifequal<T: metadata::Provider>(expressions: &[Box<expression::Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
    if expressions.len() != 4 {
        return Err(Error::ArgumentError);
    }
    let mut values : [i32; 2] = [0, 0];
    for i in 0..values.len() {
        let (val, _) = expect_integer_result!(&expressions[i], provider);
        values[i] = val;
    }
    let result = {
        if values[0] == values[1] {
            expressions[2].apply(provider)
        }
        else {
            expressions[3].apply(provider)
        }
    };
    Ok(result)
}

function_object_maker!(ifequal);
