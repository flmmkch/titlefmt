use super::super::*;
use super::super::function::Function;
use super::super::value::{ Evaluation, Value };

fn muldiv<T: metadata::Provider>(expressions: &[Box<expression::Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
    if expressions.len() != 3 {
        return Err(Error::ArgumentError);
    }
    // get all the arguments
    // first check that the last one isn't null (for the division)
    let (c, c_truth) = expect_integer_result!(&expressions[2], provider);
    if c == 0 {
        return Err(Error::ArgumentError);
    }
    let (a, a_truth) = expect_integer_result!(&expressions[0], provider);
    let (b, b_truth) = expect_integer_result!(&expressions[1], provider);
    Ok(Evaluation::new(Value::Integer((a * b) / c), a_truth | b_truth | c_truth))
}

function_object_maker!(muldiv);
