use super::super::*;
use super::super::function::Function;
use super::super::value::{ Evaluation, Value };

fn mul<T: metadata::Provider>(expressions: &[Box<expression::Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
    let mut result : i32 = 1;
    let mut truth = false;
    for expr in expressions.iter() {
        if let Some((i, expr_truth)) = try_integer_result!(expr, provider) {
            truth |= expr_truth;
            result *= i;
        }
    }
    Ok(Evaluation::new(Value::Integer(result), truth))
}

function_object_maker!(mul);
